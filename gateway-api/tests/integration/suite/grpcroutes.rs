use kube::Api;

use super::api::{GrpcRoute, GrpcRouteSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = GrpcRoute {
        spec: GrpcRouteSpec {
            hostnames: Some(vec!["example.com".into()]),
            ..Default::default()
        },
        ..Default::default()
    };

    common::crud(Api::default_namespaced(common::client().await), route).await;
}
