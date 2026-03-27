use gateway_api::experimental::xbackendtrafficpolicies::{
    XBackendTrafficPolicy, XBackendTrafficPolicySpec, XBackendTrafficPolicyTargetRefs,
};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let policy = XBackendTrafficPolicy {
        metadata: ObjectMeta {
            name: Some("test-xbackendtrafficpolicy".into()),
            ..Default::default()
        },
        spec: XBackendTrafficPolicySpec {
            target_refs: vec![XBackendTrafficPolicyTargetRefs {
                group: "".into(),
                kind: "Service".into(),
                name: "test-service".into(),
            }],
            retry_constraint: None,
            session_persistence: None,
        },
        status: None,
    };

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &policy)
        .await
        .expect("failed to create XBackendTrafficPolicy");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
