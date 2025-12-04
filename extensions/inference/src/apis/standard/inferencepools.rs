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
/// Spec defines the desired state of the InferencePool.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "inference.networking.k8s.io",
    version = "v1",
    kind = "InferencePool",
    plural = "inferencepools"
)]
#[kube(namespaced)]
#[kube(status = "InferencePoolStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct InferencePoolSpec {
    /// EndpointPickerRef is a reference to the Endpoint Picker extension and its
    /// associated configuration.
    #[serde(rename = "endpointPickerRef")]
    pub endpoint_picker_ref: ExtensionRef,
    /// Selector determines which Pods are members of this inference pool.
    /// It matches Pods by their labels only within the same namespace; cross-namespace
    /// selection is not supported.
    ///
    /// The structure of this LabelSelector is intentionally simple to be compatible
    /// with Kubernetes Service selectors, as some implementations may translate
    /// this configuration into a Service resource.
    pub selector: InferencePoolSelector,
    /// TargetPorts defines a list of ports that are exposed by this InferencePool.
    /// Currently, the list may only include a single port definition.
    #[serde(rename = "targetPorts")]
    pub target_ports: Vec<EndPointPort>,
}
/// EndpointPickerRef is a reference to the Endpoint Picker extension and its
/// associated configuration.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ExtensionRef {
    /// FailureMode configures how the parent handles the case when the Endpoint Picker extension
    /// is non-responsive. When unspecified, defaults to "FailClose".
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failureMode"
    )]
    pub failure_mode: Option<ExtensionFailureMode>,
    /// Group is the group of the referent API object. When unspecified, the default value
    /// is "", representing the Core API group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the Kubernetes resource kind of the referent.
    ///
    /// Required if the referent is ambiguous, e.g. service with multiple ports.
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
    /// Name is the name of the referent API object.
    pub name: String,
    /// Port is the port of the Endpoint Picker extension service.
    ///
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<EndPointPort>,
}
/// EndpointPickerRef is a reference to the Endpoint Picker extension and its
/// associated configuration.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum ExtensionFailureMode {
    FailOpen,
    FailClose,
}
/// Selector determines which Pods are members of this inference pool.
/// It matches Pods by their labels only within the same namespace; cross-namespace
/// selection is not supported.
///
/// The structure of this LabelSelector is intentionally simple to be compatible
/// with Kubernetes Service selectors, as some implementations may translate
/// this configuration into a Service resource.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolSelector {
    /// MatchLabels contains a set of required {key,value} pairs.
    /// An object must match every label in this map to be selected.
    /// The matching logic is an AND operation on all entries.
    #[serde(rename = "matchLabels")]
    pub match_labels: BTreeMap<String, String>,
}
/// Status defines the observed state of the InferencePool.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolStatus {
    /// Parents is a list of parent resources, typically Gateways, that are associated with
    /// the InferencePool, and the status of the InferencePool with respect to each parent.
    ///
    /// A controller that manages the InferencePool, must add an entry for each parent it manages
    /// and remove the parent entry when the controller no longer considers the InferencePool to
    /// be associated with that parent.
    ///
    /// A maximum of 32 parents will be represented in this list. When the list is empty,
    /// it indicates that the InferencePool is not associated with any parents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<InferencePoolParent>>,
}
/// ParentStatus defines the observed state of InferencePool from a Parent, i.e. Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferencePoolParent {
    /// Conditions is a list of status conditions that provide information about the observed
    /// state of the InferencePool. This field is required to be set by the controller that
    /// manages the InferencePool.
    ///
    /// Supported condition types are:
    ///
    /// * "Accepted"
    /// * "ResolvedRefs"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ParentRef is used to identify the parent resource that this status
    /// is associated with. It is used to match the InferencePool with the parent
    /// resource, such as a Gateway.
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentRef,
}
/// ParentRef is used to identify the parent resource that this status
/// is associated with. It is used to match the InferencePool with the parent
/// resource, such as a Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ParentRef {
    /// Group is the group of the referent API object. When unspecified, the referent is assumed
    /// to be in the "gateway.networking.k8s.io" API group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the kind of the referent API object. When unspecified, the referent is assumed
    /// to be a "Gateway" kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent API object.
    pub name: String,
    /// Namespace is the namespace of the referenced object. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details: <https://gateway-api.sigs.k8s.io/api-types/referencegrant/>
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
