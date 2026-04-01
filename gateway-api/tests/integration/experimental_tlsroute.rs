use gateway_api::experimental::tlsroutes::{TlsRoute, TlsRouteRules, TlsRouteSpec};
use kube::{
    Api,
    api::{DeleteParams, PostParams},
    core::ObjectMeta,
};

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;
    let api: Api<TlsRoute> = Api::default_namespaced(client.clone());

    let route = TlsRoute {
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

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create experimental TLSRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-exp-tlsroute"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.hostnames, vec!["experimental.example.com"]);
    assert_eq!(created.spec.rules.len(), 1);
    assert!(created.spec.parent_refs.is_none());

    api.delete("test-exp-tlsroute", &DeleteParams::default())
        .await
        .expect("failed to delete experimental TLSRoute");
}
