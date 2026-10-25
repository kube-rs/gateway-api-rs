use kube::Api;

use self::api::{ReferenceGrant, ReferenceGrantFrom, ReferenceGrantSpec, ReferenceGrantTo};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let grant = ReferenceGrant {
        spec: ReferenceGrantSpec {
            from: vec![ReferenceGrantFrom {
                group: "gateway.networking.k8s.io".into(),
                kind: "HTTPRoute".into(),
                namespace: "default".into(),
            }],
            to: vec![ReferenceGrantTo {
                group: "".into(),
                kind: "Service".into(),
                name: None,
            }],
        },
        ..Default::default()
    };

    common::crud(Api::default_namespaced(common::client().await), grant).await;
}
