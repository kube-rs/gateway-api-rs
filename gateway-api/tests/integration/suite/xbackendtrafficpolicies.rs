use kube::Api;

use self::api::{XBackendTrafficPolicy, XBackendTrafficPolicySpec, XBackendTrafficPolicyTargetRefs};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let policy = XBackendTrafficPolicy {
        spec: XBackendTrafficPolicySpec {
            target_refs: vec![XBackendTrafficPolicyTargetRefs {
                group: "".into(),
                kind: "Service".into(),
                name: "test-service".into(),
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    common::crud(Api::default_namespaced(common::client().await), policy).await;
}
