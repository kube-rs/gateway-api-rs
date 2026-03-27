use gateway_api::experimental::xmeshes::{XMesh, XMeshSpec};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

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

    let created = Api::all(client.clone())
        .create(&PostParams::default(), &mesh)
        .await
        .expect("failed to create XMesh");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
