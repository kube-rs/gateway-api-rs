use gateway_api::httproutes::{HTTPRoute, HttpRouteSpec};
use kube::{Api, api::PostParams, core::ObjectMeta};

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let route = HTTPRoute {
        metadata: ObjectMeta {
            name: Some("test-httproute".into()),
            ..Default::default()
        },
        spec: HttpRouteSpec::default(),
        status: None,
    };

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create HTTPRoute");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
