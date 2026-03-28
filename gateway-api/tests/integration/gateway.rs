use gateway_api::{
    constants::{GatewayConditionReason, GatewayConditionType, ListenerConditionReason, ListenerConditionType},
    gateways::{Gateway, GatewaySpec, GatewayStatus, GatewayStatusAddresses, GatewayStatusListeners},
};
use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::{Condition, Time},
    jiff::Timestamp,
};
use kube::{
    Api,
    api::{Patch, PatchParams, PostParams},
    core::ObjectMeta,
};
use serde_json::json;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud_with_status() {
    let client = common::client().await;

    let gw = Gateway {
        metadata: ObjectMeta {
            name: Some("test-gateway".into()),
            ..Default::default()
        },
        spec: GatewaySpec {
            gateway_class_name: "test-gateway-class".into(),
            ..Default::default()
        },
        status: None,
    };

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &gw)
        .await
        .expect("failed to create Gateway");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());

    let status = GatewayStatus {
        addresses: Some(vec![GatewayStatusAddresses::default()]),
        attached_listener_sets: None,
        listeners: Some(vec![GatewayStatusListeners {
            name: "tcp".into(),
            attached_routes: 0,
            supported_kinds: Some(vec![]),
            conditions: vec![Condition {
                last_transition_time: Time(Timestamp::now()),
                message: "testing gateway".into(),
                observed_generation: Some(1),
                reason: ListenerConditionReason::Programmed.to_string(),
                status: "True".into(),
                type_: ListenerConditionType::Programmed.to_string(),
            }],
        }]),
        conditions: Some(vec![Condition {
            last_transition_time: Time(Timestamp::now()),
            message: "testing gateway".into(),
            observed_generation: Some(1),
            reason: GatewayConditionReason::Programmed.to_string(),
            status: "True".into(),
            type_: GatewayConditionType::Programmed.to_string(),
        }]),
    };

    let patched: Gateway = Api::default_namespaced(client.clone())
        .patch_status(
            "test-gateway",
            &PatchParams::default(),
            &Patch::Merge(json!({ "status": status })),
        )
        .await
        .expect("failed to patch Gateway status");

    let s = patched.status.expect("status should be set");
    assert!(s.addresses.is_some());
    assert!(s.listeners.is_some());
    assert!(s.conditions.is_some());
}
