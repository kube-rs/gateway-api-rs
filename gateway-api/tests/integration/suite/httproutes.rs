use kube::Api;

use self::api::{HttpRoute, HttpRouteSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = HttpRoute {
        spec: HttpRouteSpec {
            hostnames: Some(vec!["example.com".into()]),
            ..Default::default()
        },
        ..Default::default()
    };

    common::crud(Api::default_namespaced(common::client().await), route).await;
}
