use gateway_api::experimental::referencegrants::{
    ReferenceGrant, ReferenceGrantFrom, ReferenceGrantSpec, ReferenceGrantTo,
};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let grant = ReferenceGrant {
        metadata: ObjectMeta {
            name: Some("test-exp-referencegrant".into()),
            ..Default::default()
        },
        spec: ReferenceGrantSpec {
            from: vec![ReferenceGrantFrom {
                group: "gateway.networking.k8s.io".into(),
                kind: "HTTPRoute".into(),
                namespace: "default".into(),
            }],
            to: vec![ReferenceGrantTo {
                group: "".into(),
                kind: "Service".into(),
                name: None,
            }],
        },
    };

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &grant)
        .await
        .expect("failed to create experimental ReferenceGrant");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
