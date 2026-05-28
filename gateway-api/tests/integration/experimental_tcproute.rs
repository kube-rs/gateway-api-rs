use gateway_api::experimental::tcproutes::{TcpRoute, TcpRouteRules, TcpRouteSpec};
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
    let api: Api<TcpRoute> = Api::default_namespaced(client.clone());

    let route = TcpRoute {
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

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create TCPRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-tcproute"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.rules.len(), 1);
    assert!(created.spec.rules[0].backend_refs.is_empty());
    assert!(created.spec.parent_refs.is_none());

    api.delete("test-tcproute", &DeleteParams::default())
        .await
        .expect("failed to delete TCPRoute");
}
