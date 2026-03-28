use gateway_api::experimental::udproutes::{UDPRoute, UdpRouteRules, UdpRouteSpec};
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
    let api: Api<UDPRoute> = Api::default_namespaced(client.clone());

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

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create UDPRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-udproute"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.rules.len(), 1);
    assert!(created.spec.rules[0].backend_refs.is_empty());
    assert!(created.spec.parent_refs.is_none());

    api.delete("test-udproute", &DeleteParams::default())
        .await
        .expect("failed to delete UDPRoute");
}
