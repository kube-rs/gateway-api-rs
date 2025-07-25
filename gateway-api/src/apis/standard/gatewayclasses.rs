// WARNING: generated file - manual changes will be overriden

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}
use self::prelude::*;
/// Spec defines the desired state of GatewayClass.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "GatewayClass",
    plural = "gatewayclasses"
)]
#[kube(status = "GatewayClassStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct GatewayClassSpec {
    /// ControllerName is the name of the controller that is managing Gateways of
    /// this class. The value of this field MUST be a domain prefixed path.
    ///
    /// Example: "example.net/gateway-controller".
    ///
    /// This field is not mutable and cannot be empty.
    ///
    /// Support: Core
    #[serde(rename = "controllerName")]
    pub controller_name: String,
    /// Description helps describe a GatewayClass with more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ParametersRef is a reference to a resource that contains the configuration
    /// parameters corresponding to the GatewayClass. This is optional if the
    /// controller does not require any additional configuration.
    ///
    /// ParametersRef can reference a standard Kubernetes resource, i.e. ConfigMap,
    /// or an implementation-specific custom resource. The resource can be
    /// cluster-scoped or namespace-scoped.
    ///
    /// If the referent cannot be found, refers to an unsupported kind, or when
    /// the data within that resource is malformed, the GatewayClass SHOULD be
    /// rejected with the "Accepted" status condition set to "False" and an
    /// "InvalidParameters" reason.
    ///
    /// A Gateway for this GatewayClass may provide its own `parametersRef`. When both are specified,
    /// the merging behavior is implementation specific.
    /// It is generally recommended that GatewayClass provides defaults that can be overridden by a Gateway.
    ///
    /// Support: Implementation-specific
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "parametersRef"
    )]
    pub parameters_ref: Option<GatewayClassParametersRef>,
}
/// ParametersRef is a reference to a resource that contains the configuration
/// parameters corresponding to the GatewayClass. This is optional if the
/// controller does not require any additional configuration.
///
/// ParametersRef can reference a standard Kubernetes resource, i.e. ConfigMap,
/// or an implementation-specific custom resource. The resource can be
/// cluster-scoped or namespace-scoped.
///
/// If the referent cannot be found, refers to an unsupported kind, or when
/// the data within that resource is malformed, the GatewayClass SHOULD be
/// rejected with the "Accepted" status condition set to "False" and an
/// "InvalidParameters" reason.
///
/// A Gateway for this GatewayClass may provide its own `parametersRef`. When both are specified,
/// the merging behavior is implementation specific.
/// It is generally recommended that GatewayClass provides defaults that can be overridden by a Gateway.
///
/// Support: Implementation-specific
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayClassParametersRef {
    /// Group is the group of the referent.
    pub group: String,
    /// Kind is kind of the referent.
    pub kind: String,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the referent.
    /// This field is required when referring to a Namespace-scoped resource and
    /// MUST be unset when referring to a Cluster-scoped resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
/// Status defines the current state of GatewayClass.
///
/// Implementations MUST populate status on all GatewayClass resources which
/// specify their controller name.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayClassStatus {
    /// Conditions is the current status from the controller for
    /// this GatewayClass.
    ///
    /// Controllers should prefer to publish conditions using values
    /// of GatewayClassConditionType for the type of each Condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}
