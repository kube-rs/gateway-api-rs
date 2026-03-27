use gateway_api::experimental::tcproutes::{TCPRoute, TcpRouteRules, TcpRouteSpec};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let route = TCPRoute {
        metadata: ObjectMeta {
            name: Some("test-tcproute".into()),
            ..Default::default()
        },
        spec: TcpRouteSpec {
            rules: vec![TcpRouteRules {
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
        .expect("failed to create TCPRoute");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
