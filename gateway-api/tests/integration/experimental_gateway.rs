use gateway_api::experimental::gateways::{Gateway, GatewaySpec};
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
    let api: Api<Gateway> = Api::default_namespaced(client.clone());

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

    let created = api
        .create(&PostParams::default(), &gw)
        .await
        .expect("failed to create experimental Gateway");

    assert_eq!(created.metadata.name.as_deref(), Some("test-exp-gateway"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.gateway_class_name, "test-exp-gateway-class");

    api.delete("test-exp-gateway", &DeleteParams::default())
        .await
        .expect("failed to delete experimental Gateway");
}
