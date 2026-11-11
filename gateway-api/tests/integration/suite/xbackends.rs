use kube::Api;

use self::api::{XBackend, XBackendExternalHostname, XBackendPort, XBackendSpec, XBackendType};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let backend = XBackend {
        spec: XBackendSpec {
            r#type: XBackendType::ExternalHostname,
            external_hostname: Some(XBackendExternalHostname {
                hostname: "backend.example.com".into(),
            }),
            port: XBackendPort { number: Some(443) },
            ..Default::default()
        },
        ..Default::default()
    };

    common::crud(Api::default_namespaced(common::client().await), backend).await;
}
