use gateway_api::backendtlspolicies::{
    BackendTlsPolicy, BackendTlsPolicySpec, BackendTlsPolicyTargetRefs, BackendTlsPolicyValidation,
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
    let api: Api<BackendTlsPolicy> = Api::default_namespaced(client.clone());

    let policy = BackendTlsPolicy {
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

    let created = api
        .create(&PostParams::default(), &policy)
        .await
        .expect("failed to create BackendTLSPolicy");

    assert_eq!(created.metadata.name.as_deref(), Some("test-backendtlspolicy"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.target_refs.len(), 1);
    assert_eq!(created.spec.target_refs[0].kind, "Service");
    assert_eq!(created.spec.target_refs[0].name, "test-service");
    assert_eq!(created.spec.validation.hostname, "example.com");

    api.delete("test-backendtlspolicy", &DeleteParams::default())
        .await
        .expect("failed to delete BackendTLSPolicy");
}

#[ignore]
#[tokio::test]
async fn v1alpha3() {
    use gateway_api::backendtlspolicies::v1alpha3;

    let client = common::client().await;
    let api: Api<v1alpha3::BackendTlsPolicy> = Api::default_namespaced(client.clone());

    let policy = v1alpha3::BackendTlsPolicy {
        metadata: ObjectMeta {
            name: Some("test-backendtlspolicy-v1alpha3".into()),
            ..Default::default()
        },
        spec: v1alpha3::BackendTlsPolicySpec {
            options: None,
            target_refs: vec![v1alpha3::BackendTlsPolicyTargetRefs {
                group: "".into(),
                kind: "Service".into(),
                name: "test-service".into(),
                ..Default::default()
            }],
            validation: v1alpha3::BackendTlsPolicyValidation {
                hostname: "example.com".into(),
                ..Default::default()
            },
        },
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &policy)
        .await
        .expect("failed to create v1alpha3 BackendTLSPolicy");

    assert_eq!(created.metadata.name.as_deref(), Some("test-backendtlspolicy-v1alpha3"));
    assert!(created.metadata.uid.is_some());

    let read = Api::<BackendTlsPolicy>::default_namespaced(client)
        .get("test-backendtlspolicy-v1alpha3")
        .await
        .expect("failed to read BackendTLSPolicy as v1");
    assert_eq!(read.spec.target_refs[0].name, "test-service");
    assert_eq!(read.spec.validation.hostname, "example.com");

    api.delete("test-backendtlspolicy-v1alpha3", &DeleteParams::default())
        .await
        .expect("failed to delete v1alpha3 BackendTLSPolicy");
}
