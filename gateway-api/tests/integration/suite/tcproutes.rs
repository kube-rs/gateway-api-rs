use kube::Api;

use super::api::{TcpRoute, TcpRouteRules, TcpRouteRulesBackendRefs, TcpRouteSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = TcpRoute {
        spec: TcpRouteSpec {
            rules: vec![TcpRouteRules {
                backend_refs: vec![TcpRouteRulesBackendRefs {
                    name: "test-service".into(),
                    port: Some(5432),
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
