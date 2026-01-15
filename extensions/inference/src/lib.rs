pub mod apis;

#[cfg(feature = "standard")]
pub use apis::standard::*;

#[cfg(feature = "experimental")]
pub use apis::experimental;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::process::Command;

    use anyhow::{Error, Ok};
    use hyper_util::client::legacy::Client as HTTPClient;
    use hyper_util::rt::TokioExecutor;
    use kube::Client as KubeClient;
    use kube::api::PostParams;
    use kube::config::{KubeConfigOptions, Kubeconfig};
    use kube::core::ObjectMeta;
    use kube::{Api, Config, CustomResourceExt, client::ConfigExt};
    use tower::BoxError;
    use tower::ServiceBuilder;
    use uuid::Uuid;

    use crate::apis::standard::common::EndPointPort;
    use crate::apis::standard::inferencepools::{
        ExtensionFailureMode, ExtensionRef, InferencePool, InferencePoolSelector, InferencePoolSpec,
    };

    const DEFAULT_INFERENCE_EXT_VERSION: &str = "v1.0.2";

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

        // CRDS were not reflecting in kubernetes api server, hence wait was added
        std::thread::sleep(std::time::Duration::from_secs(2));
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

        // CRDS were not reflecting in kubernetes api server, hence wait was added
        std::thread::sleep(std::time::Duration::from_secs(2));
        test_resource_deployment(client).await?;

        println!("cleaning up kind cluster {}", cluster.name);

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Test Resources
    // -------------------------------------------------------------------------

    async fn test_resource_deployment(client: kube::Client) -> Result<(), Error> {
        let mut inf_pool = InferencePool {
            metadata: ObjectMeta::default(),
            spec: InferencePoolSpec {
                endpoint_picker_ref: ExtensionRef {
                    failure_mode: Some(ExtensionFailureMode::FailClose),
                    group: None,
                    kind: None,
                    name: String::from("test-extension-name"),
                    port: Some(EndPointPort { number: 80 }),
                },
                selector: InferencePoolSelector {
                    match_labels: BTreeMap::from([(String::from("app"), String::from("test-app"))]),
                },
                target_ports: vec![EndPointPort { number: 80 }],
            },
            status: None,
        };
        inf_pool.metadata.name = Some("test-inference-pool".to_string());

        inf_pool = Api::default_namespaced(client.clone())
            .create(&PostParams::default(), &inf_pool)
            .await?;

        assert!(inf_pool.metadata.name.is_some());
        assert!(inf_pool.metadata.uid.is_some());
        assert!(inf_pool.spec.endpoint_picker_ref.failure_mode.is_some());
        assert!(inf_pool.spec.endpoint_picker_ref.port.is_some());
        assert!(inf_pool.spec.selector.match_labels.len() > 0);

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
        let version = std::env::var("GATEWAY_INFERENCE_EXT_VERSION")
            .unwrap_or_else(|_| DEFAULT_INFERENCE_EXT_VERSION.to_string());

        let semver_pattern = regex::Regex::new(r"^v?\d+\.\d+\.\d+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$")
            .map_err(|e| Error::msg(format!("Failed to compile regex: {}", e)))?;
        if !semver_pattern.is_match(&version) {
            return Err(Error::msg(format!(
                "GATEWAY_INFERENCE_EXT_VERSION '{}' is not a valid semver version",
                version
            )));
        }

        let kubeconfig_yaml = get_kind_kubeconfig(cluster_name)?;
        let temp_dir = std::env::temp_dir();
        let kubeconfig_path = temp_dir.join(format!("kubeconfig-{}", cluster_name));
        std::fs::write(&kubeconfig_path, kubeconfig_yaml)?;

        let url = format!(
            "https://github.com/kubernetes-sigs/gateway-api-inference-extension/releases/download/{}/manifests.yaml",
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
        let mut inf_pool_crd = InferencePool::crd();
        inf_pool_crd.metadata.annotations = Some(std::collections::BTreeMap::from([(
            "api-approved.kubernetes.io".to_string(),
            "https://github.com/kubernetes-sigs/gateway-api-inference-extension/pull/1173"
                .to_string(),
        )]));

        Api::all(client.clone())
            .create(&PostParams::default(), &inf_pool_crd)
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
