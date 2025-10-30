pub mod duration;
pub use duration::Duration;
pub mod apis;

#[cfg(feature = "standard")]
pub use apis::standard::*;

#[cfg(feature = "experimental")]
pub use apis::experimental;

#[cfg(test)]
mod tests {
    use std::process::Command;

    use anyhow::{Error, Ok};
    use hyper_util::client::legacy::Client as HTTPClient;
    use hyper_util::rt::TokioExecutor;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::{Condition, Time};
    use k8s_openapi::chrono::Utc;
    use kube::Client as KubeClient;
    use kube::api::{Patch, PatchParams, PostParams};
    use kube::config::{KubeConfigOptions, Kubeconfig};
    use kube::core::ObjectMeta;
    use kube::{Api, Config, CustomResourceExt, client::ConfigExt};
    use serde_json::json;
    use tower::BoxError;
    use tower::ServiceBuilder;
    use uuid::Uuid;

    use crate::{
        apis::standard::common::{ParentReference, ParentRouteStatus, RouteStatus},
        apis::standard::constants::{
            GatewayConditionReason, GatewayConditionType, ListenerConditionReason,
            ListenerConditionType, RouteConditionReason, RouteConditionType,
        },
        apis::standard::gatewayclasses::{GatewayClass, GatewayClassSpec},
        apis::standard::gateways::{
            Gateway, GatewayListeners, GatewaySpec, GatewayStatus, GatewayStatusAddresses,
            GatewayStatusListeners,
        },
        apis::standard::grpcroutes::GrpcRouteSpec,
        apis::standard::httproutes::HttpRouteSpec,
        apis::standard::referencegrants::{
            ReferenceGrantFrom, ReferenceGrantSpec, ReferenceGrantTo,
        },
    };

    const DEFAULT_GATEWAY_API_VERSION: &str = "v1.4.0";

    // -------------------------------------------------------------------------
    // Tests
    // -------------------------------------------------------------------------

    #[ignore]
    #[tokio::test]
    async fn test_deploy_resources() -> Result<(), Error> {
        let (client, cluster) = get_client(false).await?;
        let info = client.apiserver_version().await?;

        println!(
            "kind cluster {} is running, server version: {}",
            cluster.name, info.git_version
        );

        test_resource_deployment(client).await?;

        println!("cleaning up kind cluster {}", cluster.name);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_deploy_resources_upstream_crds() -> Result<(), Error> {
        let (client, cluster) = get_client(true).await?;
        let info = client.apiserver_version().await?;

        println!(
            "kind cluster {} is running, server version: {}",
            cluster.name, info.git_version
        );

        test_resource_deployment(client).await?;

        println!("cleaning up kind cluster {}", cluster.name);

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Test Resources
    // -------------------------------------------------------------------------

    async fn test_resource_deployment(client: kube::Client) -> Result<(), Error> {
        let mut gwc = GatewayClass {
            metadata: ObjectMeta::default(),
            spec: GatewayClassSpec {
                controller_name: "example.com/gateway-controller".to_string(),
                description: None,
                parameters_ref: None,
            },
            status: None,
        };
        gwc.metadata.name = Some("test-gateway-class".to_string());
        gwc = Api::all(client.clone())
            .create(&PostParams::default(), &gwc)
            .await?;

        assert!(gwc.metadata.name.is_some());
        assert!(gwc.metadata.uid.is_some());

        let mut gw = Gateway {
            metadata: ObjectMeta::default(),
            spec: GatewaySpec {
                gateway_class_name: gwc
                    .metadata
                    .name
                    .ok_or(Error::msg("could not find GatewayClass name"))?,
                listeners: vec![GatewayListeners {
                    name: "http".to_string(),
                    port: 80,
                    protocol: "HTTP".to_string(),
                    hostname: None,
                    allowed_routes: None,
                    tls: None,
                }],
                addresses: None,
                infrastructure: None,
            },
            status: None,
        };
        gw.metadata.name = Some("test-gateway".to_string());
        gw = Api::default_namespaced(client.clone())
            .create(&PostParams::default(), &gw)
            .await?;

        assert!(gw.metadata.name.is_some());
        assert!(gw.metadata.uid.is_some());

        let gw_status = GatewayStatus {
            addresses: Some(vec![GatewayStatusAddresses {
                r#type: Some("IPAddress".to_string()),
                value: "10.0.0.1".to_string(),
            }]),
            listeners: Some(vec![GatewayStatusListeners {
                name: "http".into(),
                attached_routes: 0,
                supported_kinds: vec![],
                conditions: vec![Condition {
                    last_transition_time: Time(Utc::now()),
                    message: "testing gateway".to_string(),
                    observed_generation: Some(1),
                    reason: ListenerConditionReason::Programmed.to_string(),
                    status: "True".to_string(),
                    type_: ListenerConditionType::Programmed.to_string(),
                }],
            }]),
            conditions: Some(vec![Condition {
                last_transition_time: Time(Utc::now()),
                message: "testing gateway".to_string(),
                observed_generation: Some(1),
                reason: GatewayConditionReason::Programmed.to_string(),
                status: "True".to_string(),
                type_: GatewayConditionType::Programmed.to_string(),
            }]),
        };

        gw = Api::default_namespaced(client.clone())
            .patch_status(
                gw.metadata.name.clone().unwrap().as_str(),
                &PatchParams::default(),
                &Patch::Merge(json!({
                    "status": Some(gw_status)
                })),
            )
            .await?;

        assert!(gw.status.is_some());
        assert!(gw.status.clone().unwrap().addresses.is_some());
        assert!(gw.status.clone().unwrap().listeners.is_some());
        assert!(gw.status.clone().unwrap().conditions.is_some());

        let mut http_route = crate::apis::standard::httproutes::HTTPRoute {
            metadata: ObjectMeta::default(),
            spec: HttpRouteSpec {
                hostnames: Some(vec!["example.com".to_string()]),
                parent_refs: Some(vec![ParentReference {
                    group: Some("gateway.networking.k8s.io".to_string()),
                    kind: Some("Gateway".to_string()),
                    namespace: Some("default".to_string()),
                    name: gw.metadata.name.clone().unwrap(),
                    section_name: None,
                    port: None,
                }]),
                rules: Some(vec![]),
            },
            status: None,
        };
        http_route.metadata.name = Some("test-http-route".to_string());
        http_route = Api::default_namespaced(client.clone())
            .create(&PostParams::default(), &http_route)
            .await?;

        assert!(http_route.metadata.name.is_some());
        assert!(http_route.metadata.uid.is_some());
        assert!(http_route.spec.hostnames.is_some());
        assert!(http_route.spec.parent_refs.is_some());

        let http_route_status = RouteStatus {
            parents: vec![ParentRouteStatus {
                parent_ref: ParentReference {
                    group: Some("gateway.networking.k8s.io".to_string()),
                    kind: Some("Gateway".to_string()),
                    namespace: Some("default".to_string()),
                    name: gw.metadata.name.clone().unwrap(),
                    section_name: None,
                    port: None,
                },
                controller_name: "example.com/gateway-controller".to_string(),
                conditions: vec![Condition {
                    last_transition_time: Time(Utc::now()),
                    message: "testing http route".to_string(),
                    observed_generation: Some(1),
                    reason: RouteConditionReason::Accepted.to_string(),
                    status: "True".to_string(),
                    type_: RouteConditionType::Accepted.to_string(),
                }],
            }],
        };

        http_route = Api::default_namespaced(client.clone())
            .patch_status(
                http_route.metadata.name.clone().unwrap().as_str(),
                &PatchParams::default(),
                &Patch::Merge(json!({
                    "status": Some(http_route_status)
                })),
            )
            .await?;

        assert!(http_route.status.is_some());
        assert!(!http_route.status.clone().unwrap().parents.is_empty());

        let mut grpc_route = crate::apis::standard::grpcroutes::GRPCRoute {
            metadata: ObjectMeta::default(),
            spec: GrpcRouteSpec {
                hostnames: Some(vec!["grpc.example.com".to_string()]),
                parent_refs: Some(vec![ParentReference {
                    group: Some("gateway.networking.k8s.io".to_string()),
                    kind: Some("Gateway".to_string()),
                    namespace: Some("default".to_string()),
                    name: gw.metadata.name.clone().unwrap(),
                    section_name: None,
                    port: None,
                }]),
                rules: Some(vec![]),
            },
            status: None,
        };
        grpc_route.metadata.name = Some("test-grpc-route".to_string());
        grpc_route = Api::default_namespaced(client.clone())
            .create(&PostParams::default(), &grpc_route)
            .await?;

        assert!(grpc_route.metadata.name.is_some());
        assert!(grpc_route.metadata.uid.is_some());
        assert!(grpc_route.spec.hostnames.is_some());
        assert!(grpc_route.spec.parent_refs.is_some());

        let grpc_route_status = RouteStatus {
            parents: vec![ParentRouteStatus {
                parent_ref: ParentReference {
                    group: Some("gateway.networking.k8s.io".to_string()),
                    kind: Some("Gateway".to_string()),
                    namespace: Some("default".to_string()),
                    name: gw.metadata.name.clone().unwrap(),
                    section_name: None,
                    port: None,
                },
                controller_name: "example.com/gateway-controller".to_string(),
                conditions: vec![Condition {
                    last_transition_time: Time(Utc::now()),
                    message: "testing grpc route".to_string(),
                    observed_generation: Some(1),
                    reason: RouteConditionReason::Accepted.to_string(),
                    status: "True".to_string(),
                    type_: RouteConditionType::Accepted.to_string(),
                }],
            }],
        };

        grpc_route = Api::default_namespaced(client.clone())
            .patch_status(
                grpc_route.metadata.name.clone().unwrap().as_str(),
                &PatchParams::default(),
                &Patch::Merge(json!({
                    "status": Some(grpc_route_status)
                })),
            )
            .await?;

        assert!(grpc_route.status.is_some());
        assert!(!grpc_route.status.clone().unwrap().parents.is_empty());

        let mut ref_grant = crate::apis::standard::referencegrants::ReferenceGrant {
            metadata: ObjectMeta::default(),
            spec: ReferenceGrantSpec {
                from: vec![ReferenceGrantFrom {
                    group: "gateway.networking.k8s.io".to_string(),
                    kind: "HTTPRoute".to_string(),
                    namespace: "default".to_string(),
                }],
                to: vec![ReferenceGrantTo {
                    group: "".to_string(),
                    kind: "Service".to_string(),
                    name: Some("backend-service".to_string()),
                }],
            },
        };
        ref_grant.metadata.name = Some("test-reference-grant".to_string());
        ref_grant = Api::default_namespaced(client.clone())
            .create(&PostParams::default(), &ref_grant)
            .await?;

        assert!(ref_grant.metadata.name.is_some());
        assert!(ref_grant.metadata.uid.is_some());
        assert!(!ref_grant.spec.from.is_empty());
        assert_eq!(ref_grant.spec.from[0].group, "gateway.networking.k8s.io");
        assert_eq!(ref_grant.spec.from[0].kind, "HTTPRoute");
        assert_eq!(ref_grant.spec.from[0].namespace, "default");
        assert!(!ref_grant.spec.to.is_empty());
        assert_eq!(ref_grant.spec.to[0].group, "");
        assert_eq!(ref_grant.spec.to[0].kind, "Service");
        assert_eq!(
            ref_grant.spec.to[0].name,
            Some("backend-service".to_string())
        );

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Test Utilities
    // -------------------------------------------------------------------------

    struct Cluster {
        name: String,
    }

    impl Drop for Cluster {
        fn drop(&mut self) {
            if let Err(err) = delete_kind_cluster(&self.name) {
                panic!("failed to cleanup kind cluster {}: {}", self.name, err)
            }
        }
    }

    async fn get_client(upstream: bool) -> Result<(kube::Client, Cluster), Error> {
        let cluster = create_kind_cluster()?;
        let kubeconfig_yaml = get_kind_kubeconfig(&cluster.name)?;
        let kubeconfig = Kubeconfig::from_yaml(&kubeconfig_yaml)?;
        let config =
            Config::from_custom_kubeconfig(kubeconfig, &KubeConfigOptions::default()).await?;

        let https = config.rustls_https_connector()?;
        let http_client = HTTPClient::builder(TokioExecutor::new()).build(https);
        let service = ServiceBuilder::new()
            .layer(config.base_uri_layer())
            .option_layer(config.auth_layer()?)
            .map_err(BoxError::from)
            .service(http_client);

        let client = KubeClient::new(service, config.default_namespace);

        if upstream {
            deploy_crds_upstream(&cluster.name).await?;
        } else {
            deploy_crds(client.clone()).await?;
        }

        Ok((client, cluster))
    }

    async fn deploy_crds_upstream(cluster_name: &str) -> Result<(), Error> {
        let version = std::env::var("GATEWAY_API_VERSION")
            .unwrap_or_else(|_| DEFAULT_GATEWAY_API_VERSION.to_string());

        let semver_pattern = regex::Regex::new(r"^v?\d+\.\d+\.\d+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$")
            .map_err(|e| Error::msg(format!("Failed to compile regex: {}", e)))?;
        if !semver_pattern.is_match(&version) {
            return Err(Error::msg(format!(
                "GATEWAY_API_VERSION '{}' is not a valid semver version",
                version
            )));
        }

        let kubeconfig_yaml = get_kind_kubeconfig(cluster_name)?;
        let temp_dir = std::env::temp_dir();
        let kubeconfig_path = temp_dir.join(format!("kubeconfig-{}", cluster_name));
        std::fs::write(&kubeconfig_path, kubeconfig_yaml)?;

        let url = format!(
            "https://github.com/kubernetes-sigs/gateway-api/releases/download/{}/standard-install.yaml",
            version
        );

        let output = Command::new("kubectl")
            .arg("--kubeconfig")
            .arg(&kubeconfig_path)
            .arg("apply")
            .arg("-f")
            .arg(&url)
            .output()?;

        if !output.status.success() {
            return Err(Error::msg(format!(
                "Failed to apply CRDs: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    async fn deploy_crds(client: kube::Client) -> Result<(), Error> {
        let mut gwc_crd = GatewayClass::crd();
        gwc_crd.metadata.annotations = Some(std::collections::BTreeMap::from([(
            "api-approved.kubernetes.io".to_string(),
            "https://github.com/kubernetes/enhancements/pull/1111".to_string(),
        )]));

        Api::all(client.clone())
            .create(&PostParams::default(), &gwc_crd)
            .await?;

        let mut gw_crd = Gateway::crd();
        gw_crd.metadata.annotations = Some(std::collections::BTreeMap::from([(
            "api-approved.kubernetes.io".to_string(),
            "https://github.com/kubernetes/enhancements/pull/1111".to_string(),
        )]));

        Api::all(client.clone())
            .create(&PostParams::default(), &gw_crd)
            .await?;

        let mut http_route_crd = crate::apis::standard::httproutes::HTTPRoute::crd();
        http_route_crd.metadata.annotations = Some(std::collections::BTreeMap::from([(
            "api-approved.kubernetes.io".to_string(),
            "https://github.com/kubernetes/enhancements/pull/1111".to_string(),
        )]));

        Api::all(client.clone())
            .create(&PostParams::default(), &http_route_crd)
            .await?;

        let mut grpc_route_crd = crate::apis::standard::grpcroutes::GRPCRoute::crd();
        grpc_route_crd.metadata.annotations = Some(std::collections::BTreeMap::from([(
            "api-approved.kubernetes.io".to_string(),
            "https://github.com/kubernetes/enhancements/pull/1111".to_string(),
        )]));

        Api::all(client.clone())
            .create(&PostParams::default(), &grpc_route_crd)
            .await?;

        let mut ref_grant_crd = crate::apis::standard::referencegrants::ReferenceGrant::crd();
        ref_grant_crd.metadata.annotations = Some(std::collections::BTreeMap::from([(
            "api-approved.kubernetes.io".to_string(),
            "https://github.com/kubernetes/enhancements/pull/1111".to_string(),
        )]));

        Api::all(client.clone())
            .create(&PostParams::default(), &ref_grant_crd)
            .await?;

        Ok(())
    }

    fn create_kind_cluster() -> Result<Cluster, Error> {
        let cluster_name = Uuid::new_v4().to_string();

        let output = Command::new("kind")
            .arg("create")
            .arg("cluster")
            .arg("--name")
            .arg(&cluster_name)
            .output()?;

        if !output.status.success() {
            return Err(Error::msg(String::from_utf8(output.stderr)?));
        }

        Ok(Cluster { name: cluster_name })
    }

    fn delete_kind_cluster(cluster_name: &str) -> Result<(), Error> {
        let output = Command::new("kind")
            .arg("delete")
            .arg("cluster")
            .arg("--name")
            .arg(cluster_name)
            .output()?;

        if !output.status.success() {
            return Err(Error::msg(String::from_utf8(output.stderr)?));
        }

        Ok(())
    }

    fn get_kind_kubeconfig(cluster_name: &str) -> Result<String, Error> {
        let output = Command::new("kind")
            .arg("get")
            .arg("kubeconfig")
            .arg("--name")
            .arg(cluster_name)
            .output()?;

        if !output.status.success() {
            return Err(Error::msg(String::from_utf8(output.stderr)?));
        }

        Ok(String::from_utf8(output.stdout)?)
    }
}
