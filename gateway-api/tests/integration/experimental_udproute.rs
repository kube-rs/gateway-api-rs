use gateway_api::experimental::udproutes::{UDPRoute, UdpRouteRules, UdpRouteSpec};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let route = UDPRoute {
        metadata: ObjectMeta {
            name: Some("test-udproute".into()),
            ..Default::default()
        },
        spec: UdpRouteSpec {
            rules: vec![UdpRouteRules {
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
        .expect("failed to create UDPRoute");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
