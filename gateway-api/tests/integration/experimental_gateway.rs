use gateway_api::experimental::gateways::{Gateway, GatewaySpec};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let gw = Gateway {
        metadata: ObjectMeta {
            name: Some("test-exp-gateway".into()),
            ..Default::default()
        },
        spec: GatewaySpec {
            gateway_class_name: "test-exp-gateway-class".into(),
            ..Default::default()
        },
        status: None,
    };

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &gw)
        .await
        .expect("failed to create experimental Gateway");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
