use gateway_api::httproutes::{HttpRoute, HttpRouteSpec};
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
    let api: Api<HttpRoute> = Api::default_namespaced(client.clone());

    let route = HttpRoute {
        metadata: ObjectMeta {
            name: Some("test-httproute".into()),
            ..Default::default()
        },
        spec: HttpRouteSpec::default(),
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create HTTPRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-httproute"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec, HttpRouteSpec::default());

    api.delete("test-httproute", &DeleteParams::default())
        .await
        .expect("failed to delete HTTPRoute");
}
