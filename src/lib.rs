pub mod apis;

#[cfg(test)]
mod tests {
    use std::process::Command;

    use anyhow::Error;
    use kube::api::PostParams;
    use kube::client::ConfigExt;
    use kube::config::{KubeConfigOptions, Kubeconfig};
    use kube::core::ObjectMeta;
    use kube::{Api, Config, CustomResourceExt};
    use tower::ServiceBuilder;
    use uuid::Uuid;

    use crate::apis::standard::{
        gatewayclasses::{GatewayClass, GatewayClassSpec},
        gateways::{Gateway, GatewaySpec},
    };

    // -------------------------------------------------------------------------
    // Tests
    // -------------------------------------------------------------------------

    #[ignore]
    #[tokio::test]
    async fn deploy_gateway() -> Result<(), Error> {
        let (client, cluster) = get_client().await?;
        let info = client.apiserver_version().await?;

        println!(
            "kind cluster {} is running, server version: {}",
            cluster.name, info.git_version
        );

        let mut gwc = GatewayClass {
            metadata: ObjectMeta::default(),
            spec: GatewayClassSpec {
                controller_name: "test-controller".to_string(),
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
                addresses: None,
                listeners: vec![],
            },
            status: None,
        };
        gw.metadata.name = Some("test-gateway".to_string());
        gw = Api::default_namespaced(client)
            .create(&PostParams::default(), &gw)
            .await?;

        assert!(gw.metadata.name.is_some());
        assert!(gw.metadata.uid.is_some());

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
            match delete_kind_cluster(&self.name) {
                Err(err) => panic!("failed to cleanup kind cluster {}: {}", self.name, err),
                Ok(()) => {}
            }
        }
    }

    async fn get_client() -> Result<(kube::Client, Cluster), Error> {
        let cluster = create_kind_cluster()?;
        let kubeconfig_yaml = get_kind_kubeconfig(&cluster.name)?;
        let kubeconfig = Kubeconfig::from_yaml(&kubeconfig_yaml)?;
        let config =
            Config::from_custom_kubeconfig(kubeconfig, &KubeConfigOptions::default()).await?;

        let service = ServiceBuilder::new()
            .layer(config.base_uri_layer())
            .option_layer(config.auth_layer()?)
            .service(hyper::Client::builder().build(config.rustls_https_connector()?));

        let client = kube::Client::new(service, config.default_namespace);

        deploy_crds(client.clone()).await?;

        Ok((client, cluster))
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
