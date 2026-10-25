use kube::Api;

use self::api::{UdpRoute, UdpRouteRules, UdpRouteRulesBackendRefs, UdpRouteSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = UdpRoute {
        spec: UdpRouteSpec {
            rules: vec![UdpRouteRules {
                backend_refs: vec![UdpRouteRulesBackendRefs {
                    name: "test-service".into(),
                    port: Some(53),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    common::crud(Api::default_namespaced(common::client().await), route).await;
}
