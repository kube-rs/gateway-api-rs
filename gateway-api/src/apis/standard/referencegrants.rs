// WARNING: generated file - manual changes will be overriden

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}
use self::prelude::*;
/// Spec defines the desired state of ReferenceGrant.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1beta1",
    kind = "ReferenceGrant",
    plural = "referencegrants"
)]
#[kube(namespaced)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct ReferenceGrantSpec {
    /// From describes the trusted namespaces and kinds that can reference the
    /// resources described in "To". Each entry in this list MUST be considered
    /// to be an additional place that references can be valid from, or to put
    /// this another way, entries MUST be combined using OR.
    ///
    /// Support: Core
    pub from: Vec<ReferenceGrantFrom>,
    /// To describes the resources that may be referenced by the resources
    /// described in "From". Each entry in this list MUST be considered to be an
    /// additional place that references can be valid to, or to put this another
    /// way, entries MUST be combined using OR.
    ///
    /// Support: Core
    pub to: Vec<ReferenceGrantTo>,
}
/// ReferenceGrantFrom describes trusted namespaces and kinds.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ReferenceGrantFrom {
    /// Group is the group of the referent.
    /// When empty, the Kubernetes core API group is inferred.
    ///
    /// Support: Core
    pub group: String,
    /// Kind is the kind of the referent. Although implementations may support
    /// additional resources, the following types are part of the "Core"
    /// support level for this field.
    ///
    /// When used to permit a SecretObjectReference:
    ///
    /// * Gateway
    ///
    /// When used to permit a BackendObjectReference:
    ///
    /// * GRPCRoute
    /// * HTTPRoute
    /// * TCPRoute
    /// * TLSRoute
    /// * UDPRoute
    pub kind: String,
    /// Namespace is the namespace of the referent.
    ///
    /// Support: Core
    pub namespace: String,
}
/// ReferenceGrantTo describes what Kinds are allowed as targets of the
/// references.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ReferenceGrantTo {
    /// Group is the group of the referent.
    /// When empty, the Kubernetes core API group is inferred.
    ///
    /// Support: Core
    pub group: String,
    /// Kind is the kind of the referent. Although implementations may support
    /// additional resources, the following types are part of the "Core"
    /// support level for this field:
    ///
    /// * Secret when used to permit a SecretObjectReference
    /// * Service when used to permit a BackendObjectReference
    pub kind: String,
    /// Name is the name of the referent. When unspecified, this policy
    /// refers to all resources of the specified Group and Kind in the local
    /// namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
