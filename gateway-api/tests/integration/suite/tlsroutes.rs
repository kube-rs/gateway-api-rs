use kube::Api;

use self::api::{TlsRoute, TlsRouteRules, TlsRouteRulesBackendRefs, TlsRouteSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = TlsRoute {
        spec: TlsRouteSpec {
            hostnames: vec!["example.com".into()],
            rules: vec![TlsRouteRules {
                backend_refs: vec![TlsRouteRulesBackendRefs {
                    name: "test-service".into(),
                    port: Some(443),
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
