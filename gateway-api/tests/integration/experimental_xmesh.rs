use gateway_api::experimental::xmeshes::{XMesh, XMeshSpec};
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
    let api: Api<XMesh> = Api::all(client.clone());

    let mesh = XMesh {
        metadata: ObjectMeta {
            name: Some("test-xmesh".into()),
            ..Default::default()
        },
        spec: XMeshSpec {
            controller_name: "test-mesh-controller".into(),
            description: None,
            parameters_ref: None,
        },
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &mesh)
        .await
        .expect("failed to create XMesh");

    assert_eq!(created.metadata.name.as_deref(), Some("test-xmesh"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.controller_name, "test-mesh-controller");
    assert!(created.spec.description.is_none());
    assert!(created.spec.parameters_ref.is_none());

    api.delete("test-xmesh", &DeleteParams::default())
        .await
        .expect("failed to delete XMesh");
}
