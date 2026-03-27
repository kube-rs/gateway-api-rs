use gateway_api::experimental::gatewayclasses::{GatewayClass, GatewayClassSpec};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let gwc = GatewayClass {
        metadata: ObjectMeta {
            name: Some("test-exp-gateway-class".into()),
            ..Default::default()
        },
        spec: GatewayClassSpec {
            controller_name: "test-exp-controller".into(),
            description: None,
            parameters_ref: None,
        },
        status: None,
    };

    let created = Api::all(client.clone())
        .create(&PostParams::default(), &gwc)
        .await
        .expect("failed to create experimental GatewayClass");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
