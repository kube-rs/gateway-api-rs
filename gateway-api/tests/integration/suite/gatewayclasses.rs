use kube::Api;

use super::api::{GatewayClass, GatewayClassSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let class = GatewayClass {
        spec: GatewayClassSpec {
            controller_name: "example.com/gateway-api-tests".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    common::crud(Api::all(common::client().await), class).await;
}
