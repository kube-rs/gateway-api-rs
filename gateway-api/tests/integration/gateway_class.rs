use gateway_api::gatewayclasses::{GatewayClass, GatewayClassSpec};
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
    let api: Api<GatewayClass> = Api::all(client.clone());

    let gwc = GatewayClass {
        metadata: ObjectMeta {
            name: Some("test-gateway-class".into()),
            ..Default::default()
        },
        spec: GatewayClassSpec {
            controller_name: "test-controller".into(),
            description: None,
            parameters_ref: None,
        },
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &gwc)
        .await
        .expect("failed to create GatewayClass");

    assert_eq!(created.metadata.name.as_deref(), Some("test-gateway-class"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.controller_name, "test-controller");
    assert!(created.spec.description.is_none());
    assert!(created.spec.parameters_ref.is_none());

    api.delete("test-gateway-class", &DeleteParams::default())
        .await
        .expect("failed to delete GatewayClass");
}

#[ignore]
#[tokio::test]
async fn v1beta1() {
    use gateway_api::gatewayclasses::v1beta1;

    let client = common::client().await;
    let api: Api<v1beta1::GatewayClass> = Api::all(client.clone());

    let gwc = v1beta1::GatewayClass {
        metadata: ObjectMeta {
            name: Some("test-gateway-class-v1beta1".into()),
            ..Default::default()
        },
        spec: v1beta1::GatewayClassSpec {
            controller_name: "test-controller".into(),
            ..Default::default()
        },
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &gwc)
        .await
        .expect("failed to create v1beta1 GatewayClass");

    assert_eq!(created.metadata.name.as_deref(), Some("test-gateway-class-v1beta1"));
    assert!(created.metadata.uid.is_some());

    let read = Api::<GatewayClass>::all(client)
        .get("test-gateway-class-v1beta1")
        .await
        .expect("failed to read GatewayClass as v1");
    assert_eq!(read.spec.controller_name, "test-controller");

    api.delete("test-gateway-class-v1beta1", &DeleteParams::default())
        .await
        .expect("failed to delete v1beta1 GatewayClass");
}
