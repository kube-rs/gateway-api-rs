use gateway_api::listenersets::{
    ListenerSet, ListenerSetListeners, ListenerSetParentRef, ListenerSetSpec,
};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let ls = ListenerSet {
        metadata: ObjectMeta {
            name: Some("test-listenerset".into()),
            ..Default::default()
        },
        spec: ListenerSetSpec {
            parent_ref: ListenerSetParentRef {
                group: Some("gateway.networking.k8s.io".into()),
                kind: Some("Gateway".into()),
                name: "test-gateway".into(),
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

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &ls)
        .await
        .expect("failed to create ListenerSet");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
