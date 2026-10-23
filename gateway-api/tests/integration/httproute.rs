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

#[ignore]
#[tokio::test]
async fn v1beta1() {
    use gateway_api::httproutes::v1beta1;

    let client = common::client().await;
    let api: Api<v1beta1::HttpRoute> = Api::default_namespaced(client.clone());

    let route = v1beta1::HttpRoute {
        metadata: ObjectMeta {
            name: Some("test-httproute-v1beta1".into()),
            ..Default::default()
        },
        spec: v1beta1::HttpRouteSpec {
            hostnames: Some(vec!["example.com".into()]),
            ..Default::default()
        },
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create v1beta1 HTTPRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-httproute-v1beta1"));
    assert!(created.metadata.uid.is_some());

    let read = Api::<HttpRoute>::default_namespaced(client)
        .get("test-httproute-v1beta1")
        .await
        .expect("failed to read HTTPRoute as v1");
    assert_eq!(read.spec.hostnames, Some(vec!["example.com".to_string()]));

    api.delete("test-httproute-v1beta1", &DeleteParams::default())
        .await
        .expect("failed to delete v1beta1 HTTPRoute");
}
