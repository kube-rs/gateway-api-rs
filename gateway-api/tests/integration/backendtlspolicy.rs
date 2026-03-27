use gateway_api::backendtlspolicies::{
    BackendTLSPolicy, BackendTlsPolicySpec, BackendTlsPolicyTargetRefs,
    BackendTlsPolicyValidation,
};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let policy = BackendTLSPolicy {
        metadata: ObjectMeta {
            name: Some("test-backendtlspolicy".into()),
            ..Default::default()
        },
        spec: BackendTlsPolicySpec {
            options: None,
            target_refs: vec![BackendTlsPolicyTargetRefs {
                group: "".into(),
                kind: "Service".into(),
                name: "test-service".into(),
                ..Default::default()
            }],
            validation: BackendTlsPolicyValidation {
                ca_certificate_refs: None,
                subject_alt_names: None,
                well_known_ca_certificates: None,
                hostname: "example.com".into(),
            },
        },
        status: None,
    };

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &policy)
        .await
        .expect("failed to create BackendTLSPolicy");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
