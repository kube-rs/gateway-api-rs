use gateway_api::grpcroutes::{GrpcRoute, GrpcRouteSpec};
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
            name: Some("test-grpcroute".into()),
            ..Default::default()
        },
        spec: GrpcRouteSpec::default(),
        status: None,
    };

    let created = api
        .create(&PostParams::default(), &route)
        .await
        .expect("failed to create GRPCRoute");

    assert_eq!(created.metadata.name.as_deref(), Some("test-grpcroute"));
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec, GrpcRouteSpec::default());

    api.delete("test-grpcroute", &DeleteParams::default())
        .await
        .expect("failed to delete GRPCRoute");
}
