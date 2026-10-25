use kube::Api;

use self::api::{XMesh, XMeshSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let mesh = XMesh {
        spec: XMeshSpec {
            controller_name: "example.com/gateway-api-tests".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    common::crud(Api::all(common::client().await), mesh).await;
}
