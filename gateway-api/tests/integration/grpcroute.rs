use gateway_api::grpcroutes::{GRPCRoute, GrpcRouteSpec};
use kube::Api;
use kube::api::PostParams;
use kube::core::ObjectMeta;

use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let client = common::client().await;

    let route = GRPCRoute {
        metadata: ObjectMeta {
            name: Some("test-grpcroute".into()),
            ..Default::default()
        },
        spec: GrpcRouteSpec::default(),
        status: None,
    };

    let created = Api::default_namespaced(client.clone())
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create GRPCRoute");

    assert!(created.metadata.name.is_some());
    assert!(created.metadata.uid.is_some());
}
