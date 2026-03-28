use gateway_api::experimental::referencegrants::{
    ReferenceGrant, ReferenceGrantFrom, ReferenceGrantSpec, ReferenceGrantTo,
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
    let api: Api<ReferenceGrant> = Api::default_namespaced(client.clone());

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

    let created = api
        .create(&PostParams::default(), &grant)
        .await
        .expect("failed to create experimental ReferenceGrant");

    assert_eq!(created.metadata.name.as_deref(), Some("test-exp-referencegrant"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.from.len(), 1);
    assert_eq!(created.spec.from[0].kind, "HTTPRoute");
    assert_eq!(created.spec.from[0].namespace, "default");
    assert_eq!(created.spec.to.len(), 1);
    assert_eq!(created.spec.to[0].kind, "Service");

    api.delete("test-exp-referencegrant", &DeleteParams::default())
        .await
        .expect("failed to delete experimental ReferenceGrant");
}
