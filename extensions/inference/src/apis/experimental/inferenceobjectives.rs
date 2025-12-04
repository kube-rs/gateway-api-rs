// WARNING: generated file - manual changes will be overriden

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube_derive::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}
use self::prelude::*;
/// InferenceObjectiveSpec represents the desired state of a specific model use case. This resource is
/// managed by the "Inference Workload Owner" persona.
///
/// The Inference Workload Owner persona is someone that trains, verifies, and
/// leverages a large language model from a model frontend, drives the lifecycle
/// and rollout of new versions of those models, and defines the specific
/// performance and latency goals for the model. These workloads are
/// expected to operate within an InferencePool sharing compute capacity with other
/// InferenceObjectives, defined by the Inference Platform Admin.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "inference.networking.x-k8s.io",
    version = "v1alpha2",
    kind = "InferenceObjective",
    plural = "inferenceobjectives"
)]
#[kube(namespaced)]
#[kube(status = "InferenceObjectiveStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct InferenceObjectiveSpec {
    /// PoolRef is a reference to the inference pool, the pool must exist in the same namespace.
    #[serde(rename = "poolRef")]
    pub pool_ref: InferenceObjectivePoolRef,
    /// Priority defines how important it is to serve the request compared to other requests in the same pool.
    /// Priority is an integer value that defines the priority of the request.
    /// The higher the value, the more critical the request is; negative values _are_ allowed.
    /// No default value is set for this field, allowing for future additions of new fields that may 'one of' with this field.
    /// However, implementations that consume this field (such as the Endpoint Picker) will treat an unset value as '0'.
    /// Priority is used in flow control, primarily in the event of resource scarcity(requests need to be queued).
    /// All requests will be queued, and flow control will _always_ allow requests of higher priority to be served first.
    /// Fairness is only enforced and tracked between requests of the same priority.
    ///
    /// Example: requests with Priority 10 will always be served before
    /// requests with Priority of 0 (the value used if Priority is unset or no InfereneceObjective is specified).
    /// Similarly requests with a Priority of -10 will always be served after requests with Priority of 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}
/// PoolRef is a reference to the inference pool, the pool must exist in the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferenceObjectivePoolRef {
    /// Group is the group of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent. For example "InferencePool".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
}
/// InferenceObjectiveStatus defines the observed state of InferenceObjective
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct InferenceObjectiveStatus {
    /// Conditions track the state of the InferenceObjective.
    ///
    /// Known condition types are:
    ///
    /// * "Accepted"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}
