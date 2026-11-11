use kube::Api;

use self::api::{
    XTelemetryPolicy, XTelemetryPolicySpec, XTelemetryPolicyTargetRefs, XTelemetryPolicyTracing,
    XTelemetryPolicyTracingMode, XTelemetryPolicyTracingProvider, XTelemetryPolicyTracingProviderBackendRef,
};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let policy = XTelemetryPolicy {
        spec: XTelemetryPolicySpec {
            target_refs: vec![XTelemetryPolicyTargetRefs {
                group: "gateway.networking.k8s.io".into(),
                kind: "Gateway".into(),
                name: "test-gateway".into(),
            }],
            // An enabled mode requires a provider.
            tracing: Some(XTelemetryPolicyTracing {
                mode: Some(XTelemetryPolicyTracingMode::Enabled),
                provider: Some(XTelemetryPolicyTracingProvider {
                    backend_ref: XTelemetryPolicyTracingProviderBackendRef {
                        name: "test-collector".into(),
                        port: Some(4317),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
        },
        ..Default::default()
    };

    common::crud(Api::default_namespaced(common::client().await), policy).await;
}
