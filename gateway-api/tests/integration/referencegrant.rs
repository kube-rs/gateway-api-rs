use gateway_api::referencegrants::{ReferenceGrant, ReferenceGrantFrom, ReferenceGrantSpec, ReferenceGrantTo};
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
    let api: Api<ReferenceGrant> = Api::default_namespaced(client.clone());

    let grant = ReferenceGrant {
        metadata: ObjectMeta {
            name: Some("test-referencegrant".into()),
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

    let created = api
        .create(&PostParams::default(), &grant)
        .await
        .expect("failed to create ReferenceGrant");

    assert_eq!(created.metadata.name.as_deref(), Some("test-referencegrant"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.from.len(), 1);
    assert_eq!(created.spec.from[0].kind, "HTTPRoute");
    assert_eq!(created.spec.from[0].namespace, "default");
    assert_eq!(created.spec.to.len(), 1);
    assert_eq!(created.spec.to[0].kind, "Service");

    api.delete("test-referencegrant", &DeleteParams::default())
        .await
        .expect("failed to delete ReferenceGrant");
}

#[ignore]
#[tokio::test]
async fn v1beta1() {
    use gateway_api::referencegrants::v1beta1;

    let client = common::client().await;
    let api: Api<v1beta1::ReferenceGrant> = Api::default_namespaced(client.clone());

    let grant = v1beta1::ReferenceGrant {
        metadata: ObjectMeta {
            name: Some("test-referencegrant-v1beta1".into()),
            ..Default::default()
        },
        spec: v1beta1::ReferenceGrantSpec {
            from: vec![v1beta1::ReferenceGrantFrom {
                group: "gateway.networking.k8s.io".into(),
                kind: "HTTPRoute".into(),
                namespace: "default".into(),
            }],
            to: vec![v1beta1::ReferenceGrantTo {
                group: "".into(),
                kind: "Service".into(),
                name: None,
            }],
        },
    };

    let created = api
        .create(&PostParams::default(), &grant)
        .await
        .expect("failed to create v1beta1 ReferenceGrant");

    assert_eq!(created.metadata.name.as_deref(), Some("test-referencegrant-v1beta1"));
    assert!(created.metadata.uid.is_some());

    let read = Api::<ReferenceGrant>::default_namespaced(client)
        .get("test-referencegrant-v1beta1")
        .await
        .expect("failed to read ReferenceGrant as v1");
    assert_eq!(read.spec.from[0].kind, "HTTPRoute");
    assert_eq!(read.spec.to[0].kind, "Service");

    api.delete("test-referencegrant-v1beta1", &DeleteParams::default())
        .await
        .expect("failed to delete v1beta1 ReferenceGrant");
}
