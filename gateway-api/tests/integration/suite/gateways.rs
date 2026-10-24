use gateway_api::constants::{
    GatewayConditionReason, GatewayConditionType, ListenerConditionReason, ListenerConditionType,
};
use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::{Condition, Time},
    jiff::Timestamp,
};
use kube::{
    Api, ResourceExt,
    api::{DeleteParams, Patch, PatchParams, PostParams},
};
use serde_json::json;

use super::api::{Gateway, GatewayListeners, GatewaySpec, GatewayStatus, GatewayStatusListeners};
use crate::common;

fn gateway() -> Gateway {
    Gateway {
        spec: GatewaySpec {
            gateway_class_name: "test-gateway-class".into(),
            listeners: vec![GatewayListeners {
                name: "http".into(),
                port: 80,
                protocol: "HTTP".into(),
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    }
}

#[ignore]
#[tokio::test]
async fn crud() {
    common::crud(Api::default_namespaced(common::client().await), gateway()).await;
}

#[ignore]
#[tokio::test]
async fn status() {
    let api: Api<Gateway> = Api::default_namespaced(common::client().await);
    let mut gw = gateway();
    gw.metadata.generate_name = Some("test-".into());
    let created = api
        .create(&PostParams::default(), &gw)
        .await
        .expect("failed to create Gateway");
    let name = created.name_any();

    let condition = |type_: String, reason: String| Condition {
        last_transition_time: Time(Timestamp::now()),
        message: "testing gateway".into(),
        observed_generation: Some(1),
        reason,
        status: "True".into(),
        type_,
    };
    let status = GatewayStatus {
        listeners: Some(vec![GatewayStatusListeners {
            name: "http".into(),
            attached_routes: 0,
            supported_kinds: Some(vec![]),
            conditions: vec![condition(
                ListenerConditionType::Programmed.to_string(),
                ListenerConditionReason::Programmed.to_string(),
            )],
        }]),
        conditions: Some(vec![condition(
            GatewayConditionType::Programmed.to_string(),
            GatewayConditionReason::Programmed.to_string(),
        )]),
        ..Default::default()
    };

    let patched = api
        .patch_status(
            &name,
            &PatchParams::default(),
            &Patch::Merge(json!({ "status": status })),
        )
        .await;
    api.delete(&name, &DeleteParams::default())
        .await
        .expect("failed to delete Gateway");

    let s = patched
        .expect("failed to patch Gateway status")
        .status
        .expect("status should be set");
    let listeners = s.listeners.expect("listeners should be set");
    assert_eq!(listeners.len(), 1);
    assert_eq!(listeners[0].name, "http");
    let conditions = s.conditions.expect("conditions should be set");
    assert_eq!(conditions.len(), 1);
    assert_eq!(conditions[0].reason, GatewayConditionReason::Programmed.to_string());
    assert_eq!(conditions[0].status, "True");
}
