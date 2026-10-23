use std::time::Duration;

use anyhow::{Result, bail};
use hyper_util::{client::legacy::Client as HttpClient, rt::TokioExecutor};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    Api, Client, Config, CustomResourceExt,
    api::PostParams,
    client::ConfigExt,
    config::{KubeConfigOptions, Kubeconfig},
    core::crd::merge_crds,
};
use tokio::sync::OnceCell;
use tower::{BoxError, ServiceBuilder};

// -----------------------------------------------------------------------------
// Shared cluster state
// -----------------------------------------------------------------------------

static CLIENT_INIT: OnceCell<String> = OnceCell::const_new();

/// Return a fresh client bound to the current tokio runtime.
/// The cluster is expected to already exist — `KUBECONFIG` env var must point
/// at a valid kubeconfig file. CRDs are deployed once on first call.
pub async fn client() -> Client {
    let kubeconfig = CLIENT_INIT
        .get_or_init(|| async {
            let path = std::env::var("KUBECONFIG").expect("KUBECONFIG env var must be set for integration tests");
            let kubeconfig =
                std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read KUBECONFIG at {path}: {e}"));

            let client = make_client(&kubeconfig).await.expect("failed to create kube client");

            deploy_standard_crds(&client)
                .await
                .expect("failed to deploy standard CRDs");

            #[cfg(feature = "experimental")]
            deploy_experimental_crds(&client)
                .await
                .expect("failed to deploy experimental CRDs");

            kubeconfig
        })
        .await;

    make_client(kubeconfig).await.expect("failed to create kube client")
}

// -----------------------------------------------------------------------------
// Client construction
// -----------------------------------------------------------------------------

async fn make_client(kubeconfig_yaml: &str) -> Result<Client> {
    let kubeconfig = Kubeconfig::from_yaml(kubeconfig_yaml)?;
    let config = Config::from_custom_kubeconfig(kubeconfig, &KubeConfigOptions::default()).await?;

    let https = config.rustls_https_connector()?;
    let http_client = HttpClient::builder(TokioExecutor::new()).build(https);
    let service = ServiceBuilder::new()
        .layer(config.base_uri_layer())
        .option_layer(config.auth_layer()?)
        .map_err(BoxError::from)
        .service(http_client);

    Ok(Client::new(service, config.default_namespace))
}

// -----------------------------------------------------------------------------
// CRD deployment
// -----------------------------------------------------------------------------

async fn deploy_crd(client: &Client, mut crd: CustomResourceDefinition) -> Result<()> {
    let name = crd.metadata.name.clone().unwrap_or_default();
    crd.metadata.annotations = Some(std::collections::BTreeMap::from([(
        "api-approved.kubernetes.io".to_string(),
        "https://github.com/kubernetes/enhancements/pull/1111".to_string(),
    )]));
    let api: Api<CustomResourceDefinition> = Api::all(client.clone());
    api.create(&PostParams::default(), &crd).await?;

    // Wait for the CRD to be accepted by the API server
    for _ in 0..30 {
        let c = api.get(&name).await?;
        let established = c
            .status
            .as_ref()
            .and_then(|s| s.conditions.as_ref())
            .and_then(|conds| conds.iter().find(|c| c.type_ == "Established"))
            .is_some_and(|c| c.status == "True");
        if established {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    bail!("timed out waiting for CRD {name} to become established");
}

/// Deploys one CRD serving every API version the crate generates. Unlike the
/// upstream CRDs, every version is served, so each binding can be exercised.
async fn deploy_merged_crd(client: &Client, crds: Vec<CustomResourceDefinition>, storage: &str) -> Result<()> {
    deploy_crd(client, merge_crds(crds, storage)?).await
}

async fn deploy_standard_crds(client: &Client) -> Result<()> {
    use gateway_api::{
        backendtlspolicies, gatewayclasses, gateways, grpcroutes::GrpcRoute, httproutes, listenersets::ListenerSet,
        referencegrants, tlsroutes,
    };

    deploy_merged_crd(
        client,
        vec![
            backendtlspolicies::v1::BackendTlsPolicy::crd(),
            backendtlspolicies::v1alpha3::BackendTlsPolicy::crd(),
        ],
        "v1",
    )
    .await?;
    deploy_merged_crd(
        client,
        vec![
            gatewayclasses::v1::GatewayClass::crd(),
            gatewayclasses::v1beta1::GatewayClass::crd(),
        ],
        "v1",
    )
    .await?;
    deploy_merged_crd(
        client,
        vec![gateways::v1::Gateway::crd(), gateways::v1beta1::Gateway::crd()],
        "v1",
    )
    .await?;
    deploy_crd(client, GrpcRoute::crd()).await?;
    deploy_merged_crd(
        client,
        vec![httproutes::v1::HttpRoute::crd(), httproutes::v1beta1::HttpRoute::crd()],
        "v1",
    )
    .await?;
    deploy_crd(client, ListenerSet::crd()).await?;
    deploy_merged_crd(
        client,
        vec![
            referencegrants::v1::ReferenceGrant::crd(),
            referencegrants::v1beta1::ReferenceGrant::crd(),
        ],
        "v1beta1",
    )
    .await?;
    deploy_merged_crd(
        client,
        vec![
            tlsroutes::v1::TlsRoute::crd(),
            tlsroutes::v1alpha2::TlsRoute::crd(),
            tlsroutes::v1alpha3::TlsRoute::crd(),
        ],
        "v1",
    )
    .await?;
    Ok(())
}

#[cfg(feature = "experimental")]
async fn deploy_experimental_crds(client: &Client) -> Result<()> {
    use gateway_api::experimental::{
        tcproutes::TcpRoute, udproutes::UdpRoute, xbackendtrafficpolicies::XBackendTrafficPolicy, xmeshes::XMesh,
    };

    deploy_crd(client, TcpRoute::crd()).await?;
    deploy_crd(client, UdpRoute::crd()).await?;
    deploy_crd(client, XBackendTrafficPolicy::crd()).await?;
    deploy_crd(client, XMesh::crd()).await?;
    Ok(())
}
