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
    api::{DeleteParams, Patch, PatchParams, PostParams},
    core::ObjectMeta,
};
use serde_json::json;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud_with_status() {
    let client = common::client().await;
    let api: Api<Gateway> = Api::default_namespaced(client.clone());

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

    let created = api
        .create(&PostParams::default(), &gw)
        .await
        .expect("failed to create Gateway");

    assert_eq!(created.metadata.name.as_deref(), Some("test-gateway"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.gateway_class_name, "test-gateway-class");

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

    let patched: Gateway = api
        .patch_status(
            "test-gateway",
            &PatchParams::default(),
            &Patch::Merge(json!({ "status": status })),
        )
        .await
        .expect("failed to patch Gateway status");

    let s = patched.status.expect("status should be set");
    assert_eq!(s.addresses.as_ref().map(|a| a.len()), Some(1));
    let listeners = s.listeners.as_ref().expect("listeners should be set");
    assert_eq!(listeners.len(), 1);
    assert_eq!(listeners[0].name, "tcp");
    assert_eq!(listeners[0].attached_routes, 0);
    let conditions = s.conditions.as_ref().expect("conditions should be set");
    assert_eq!(conditions.len(), 1);
    assert_eq!(conditions[0].reason, GatewayConditionReason::Programmed.to_string());
    assert_eq!(conditions[0].status, "True");

    api.delete("test-gateway", &DeleteParams::default())
        .await
        .expect("failed to delete Gateway");
}
