use gateway_api::experimental::listenersets::{
    ListenerSet, ListenerSetListeners, ListenerSetParentRef, ListenerSetSpec,
};
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
    let api: Api<ListenerSet> = Api::default_namespaced(client.clone());

    let ls = ListenerSet {
        metadata: ObjectMeta {
            name: Some("test-exp-listenerset".into()),
            ..Default::default()
        },
        spec: ListenerSetSpec {
            parent_ref: ListenerSetParentRef {
                group: Some("gateway.networking.k8s.io".into()),
                kind: Some("Gateway".into()),
                name: "test-exp-gateway".into(),
                namespace: None,
            },
            listeners: vec![ListenerSetListeners {
                name: "http".into(),
                port: 8080,
                protocol: "HTTP".into(),
                ..Default::default()
            }],
        },
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &ls)
        .await
        .expect("failed to create experimental ListenerSet");

    assert_eq!(created.metadata.name.as_deref(), Some("test-exp-listenerset"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.parent_ref.name, "test-exp-gateway");
    assert_eq!(created.spec.listeners.len(), 1);
    assert_eq!(created.spec.listeners[0].name, "http");
    assert_eq!(created.spec.listeners[0].port, 8080);
    assert_eq!(created.spec.listeners[0].protocol, "HTTP");

    api.delete("test-exp-listenerset", &DeleteParams::default())
        .await
        .expect("failed to delete experimental ListenerSet");
}
