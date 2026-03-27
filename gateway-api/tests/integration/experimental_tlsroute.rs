use gateway_api::experimental::tlsroutes::{TLSRoute, TlsRouteRules, TlsRouteSpec};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let route = TLSRoute {
        metadata: ObjectMeta {
            name: Some("test-exp-tlsroute".into()),
            ..Default::default()
        },
        spec: TlsRouteSpec {
            hostnames: vec!["experimental.example.com".into()],
            rules: vec![TlsRouteRules {
                backend_refs: vec![],
                name: None,
            }],
            parent_refs: None,
            use_default_gateways: None,
        },
        status: None,
    };

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create experimental TLSRoute");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
