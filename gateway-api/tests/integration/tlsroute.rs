use gateway_api::tlsroutes::{TLSRoute, TlsRouteRules, TlsRouteSpec};
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
            name: Some("test-tlsroute".into()),
            ..Default::default()
        },
        spec: TlsRouteSpec {
            hostnames: vec!["example.com".into()],
            rules: vec![TlsRouteRules {
                backend_refs: vec![],
                name: None,
            }],
            parent_refs: None,
        },
        status: None,
    };

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create TLSRoute");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
