// WARNING: generated file - manual changes will be overriden

use super::common::*;
#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube_derive::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;
/// InferencePoolSpec defines the desired state of InferencePool
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "inference.networking.x-k8s.io",
    version = "v1alpha2",
    kind = "InferencePool",
    plural = "inferencepools"
)]
#[kube(namespaced)]
#[kube(status = "InferencePoolStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct InferencePoolSpec {
    /// Extension configures an endpoint picker as an extension service.
    #[serde(rename = "extensionRef")]
    pub extension_ref: ExtensionRef,
    /// Selector defines a map of labels to watch model server Pods
    /// that should be included in the InferencePool.
    /// In some cases, implementations may translate this field to a Service selector, so this matches the simple
    /// map used for Service selectors instead of the full Kubernetes LabelSelector type.
    /// If specified, it will be applied to match the model server pods in the same namespace as the InferencePool.
    /// Cross namesoace selector is not supported.
    pub selector: BTreeMap<String, String>,
    /// TargetPortNumber defines the port number to access the selected model server Pods.
    /// The number must be in the range 1 to 65535.
    #[serde(rename = "targetPortNumber")]
    pub target_port_number: i32,
}
/// Extension configures an endpoint picker as an extension service.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ExtensionRef {
    /// Configures how the gateway handles the case when the extension is not responsive.
    /// Defaults to failClose.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failureMode"
    )]
    pub failure_mode: Option<ExtensionFailureMode>,
    /// Group is the group of the referent.
    /// The default value is "", representing the Core API group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the Kubernetes resource kind of the referent.
    ///
    /// Defaults to "Service" when not specified.
    ///
    /// ExternalName services can refer to CNAME DNS records that may live
    /// outside of the cluster and as such are difficult to reason about in
    /// terms of conformance. They also may not be safe to forward to (see
    /// CVE-2021-25740 for more information). Implementations MUST NOT
    /// support ExternalName Services.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
    /// The port number on the service running the extension. When unspecified,
    /// implementations SHOULD infer a default value of 9002 when the Kind is
    /// Service.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "portNumber"
    )]
    pub port_number: Option<i32>,
}
/// Extension configures an endpoint picker as an extension service.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum ExtensionFailureMode {
    FailOpen,
    FailClose,
}
/// Status defines the observed state of InferencePool.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolStatus {
    /// Parents is a list of parent resources (usually Gateways) that are
    /// associated with the InferencePool, and the status of the InferencePool with respect to
    /// each parent.
    ///
    /// A maximum of 32 Gateways will be represented in this list. When the list contains
    /// `kind: Status, name: default`, it indicates that the InferencePool is not
    /// associated with any Gateway and a controller must perform the following:
    ///
    ///  - Remove the parent when setting the "Accepted" condition.
    ///  - Add the parent when the controller will no longer manage the InferencePool
    ///    and no other parents exist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Vec<InferencePoolParent>>,
}
/// PoolStatus defines the observed state of InferencePool from a Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolParent {
    /// Conditions track the state of the InferencePool.
    ///
    /// Known condition types are:
    ///
    /// * "Accepted"
    /// * "ResolvedRefs"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// GatewayRef indicates the gateway that observed state of InferencePool.
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentRef,
}
/// GatewayRef indicates the gateway that observed state of InferencePool.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ParentRef {
    /// Group is the group of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent. For example "Gateway".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the referent.  If not present,
    /// the namespace of the referent is assumed to be the same as
    /// the namespace of the referring object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
