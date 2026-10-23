use kube::Api;

use super::api::{BackendTlsPolicy, BackendTlsPolicySpec, BackendTlsPolicyTargetRefs, BackendTlsPolicyValidation};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let policy = BackendTlsPolicy {
        spec: BackendTlsPolicySpec {
            target_refs: vec![BackendTlsPolicyTargetRefs {
                group: "".into(),
                kind: "Service".into(),
                name: "test-service".into(),
                ..Default::default()
            }],
            validation: BackendTlsPolicyValidation {
                hostname: "example.com".into(),
                well_known_ca_certificates: Some("System".into()),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    common::crud(Api::default_namespaced(common::client().await), policy).await;
}
