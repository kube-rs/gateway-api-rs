use gateway_api::tlsroutes::{TlsRoute, TlsRouteRules, TlsRouteSpec};
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
    let api: Api<TlsRoute> = Api::default_namespaced(client.clone());

    let route = TlsRoute {
        metadata: ObjectMeta {
            name: Some("test-tlsroute".into()),
            ..Default::default()
        },
        spec: TlsRouteSpec {
            hostnames: vec!["example.com".into()],
            rules: vec![TlsRouteRules {
                backend_refs: vec![],
                name: None,
            }],
            parent_refs: None,
        },
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create TLSRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-tlsroute"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.hostnames, vec!["example.com"]);
    assert_eq!(created.spec.rules.len(), 1);
    assert!(created.spec.parent_refs.is_none());

    api.delete("test-tlsroute", &DeleteParams::default())
        .await
        .expect("failed to delete TLSRoute");
}

#[ignore]
#[tokio::test]
async fn v1alpha2() {
    use gateway_api::tlsroutes::v1alpha2;

    let client = common::client().await;
    let api: Api<v1alpha2::TlsRoute> = Api::default_namespaced(client.clone());

    let route = v1alpha2::TlsRoute {
        metadata: ObjectMeta {
            name: Some("test-tlsroute-v1alpha2".into()),
            ..Default::default()
        },
        spec: v1alpha2::TlsRouteSpec {
            hostnames: Some(vec!["example.com".into()]),
            rules: vec![v1alpha2::TlsRouteRules::default()],
            parent_refs: None,
        },
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create v1alpha2 TLSRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-tlsroute-v1alpha2"));
    assert!(created.metadata.uid.is_some());

    let read = Api::<TlsRoute>::default_namespaced(client)
        .get("test-tlsroute-v1alpha2")
        .await
        .expect("failed to read TLSRoute as v1");
    assert_eq!(read.spec.hostnames, vec!["example.com"]);

    api.delete("test-tlsroute-v1alpha2", &DeleteParams::default())
        .await
        .expect("failed to delete v1alpha2 TLSRoute");
}

#[ignore]
#[tokio::test]
async fn v1alpha3() {
    use gateway_api::tlsroutes::v1alpha3;

    let client = common::client().await;
    let api: Api<v1alpha3::TlsRoute> = Api::default_namespaced(client.clone());

    let route = v1alpha3::TlsRoute {
        metadata: ObjectMeta {
            name: Some("test-tlsroute-v1alpha3".into()),
            ..Default::default()
        },
        spec: v1alpha3::TlsRouteSpec {
            hostnames: vec!["example.com".into()],
            rules: vec![v1alpha3::TlsRouteRules::default()],
            parent_refs: None,
        },
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create v1alpha3 TLSRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-tlsroute-v1alpha3"));
    assert!(created.metadata.uid.is_some());

    let read = Api::<TlsRoute>::default_namespaced(client)
        .get("test-tlsroute-v1alpha3")
        .await
        .expect("failed to read TLSRoute as v1");
    assert_eq!(read.spec.hostnames, vec!["example.com"]);

    api.delete("test-tlsroute-v1alpha3", &DeleteParams::default())
        .await
        .expect("failed to delete v1alpha3 TLSRoute");
}
