use gateway_api::tlsroutes::{TLSRoute, TlsRouteRules, TlsRouteSpec};
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
    let api: Api<TLSRoute> = Api::default_namespaced(client.clone());

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

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create TLSRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-tlsroute"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.hostnames, vec!["example.com"]);
    assert_eq!(created.spec.rules.len(), 1);
    assert!(created.spec.parent_refs.is_none());

    api.delete("test-tlsroute", &DeleteParams::default())
        .await
        .expect("failed to delete TLSRoute");
}
