use kube::Api;

use super::api::{ListenerSet, ListenerSetListeners, ListenerSetParentRef, ListenerSetSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let set = ListenerSet {
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
        ..Default::default()
    };

    common::crud(Api::default_namespaced(common::client().await), set).await;
}
