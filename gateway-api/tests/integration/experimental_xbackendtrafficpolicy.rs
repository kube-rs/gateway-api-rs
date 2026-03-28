use gateway_api::experimental::xbackendtrafficpolicies::{
    XBackendTrafficPolicy, XBackendTrafficPolicySpec, XBackendTrafficPolicyTargetRefs,
};
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
    let api: Api<XBackendTrafficPolicy> = Api::default_namespaced(client.clone());

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

    let created = api
        .create(&PostParams::default(), &policy)
        .await
        .expect("failed to create XBackendTrafficPolicy");

    assert_eq!(created.metadata.name.as_deref(), Some("test-xbackendtrafficpolicy"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.target_refs.len(), 1);
    assert_eq!(created.spec.target_refs[0].kind, "Service");
    assert_eq!(created.spec.target_refs[0].name, "test-service");
    assert!(created.spec.retry_constraint.is_none());
    assert!(created.spec.session_persistence.is_none());

    api.delete("test-xbackendtrafficpolicy", &DeleteParams::default())
        .await
        .expect("failed to delete XBackendTrafficPolicy");
}
