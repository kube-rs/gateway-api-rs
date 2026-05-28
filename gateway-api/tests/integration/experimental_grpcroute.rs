use gateway_api::experimental::grpcroutes::{GrpcRoute, GrpcRouteSpec};
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
    let api: Api<GrpcRoute> = Api::default_namespaced(client.clone());

    let route = GrpcRoute {
        metadata: ObjectMeta {
            name: Some("test-exp-grpcroute".into()),
            ..Default::default()
        },
        spec: GrpcRouteSpec::default(),
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create experimental GRPCRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-exp-grpcroute"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec, GrpcRouteSpec::default());

    api.delete("test-exp-grpcroute", &DeleteParams::default())
        .await
        .expect("failed to delete experimental GRPCRoute");
}
