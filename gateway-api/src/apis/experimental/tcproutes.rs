// WARNING: generated file - manual changes will be overriden

use super::common::*;
#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}
use self::prelude::*;
/// Spec defines the desired state of TCPRoute.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha2",
    kind = "TCPRoute",
    plural = "tcproutes"
)]
#[kube(namespaced)]
#[kube(status = "TcpRouteStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct TcpRouteSpec {
    /// ParentRefs references the resources (usually Gateways) that a Route wants
    /// to be attached to. Note that the referenced parent resource needs to
    /// allow this for the attachment to be complete. For Gateways, that means
    /// the Gateway needs to allow attachment from Routes of this kind and
    /// namespace. For Services, that means the Service must either be in the same
    /// namespace for a "producer" route, or the mesh implementation must support
    /// and allow "consumer" routes for the referenced Service. ReferenceGrant is
    /// not applicable for governing ParentRefs to Services - it is not possible to
    /// create a "producer" route for a Service in a different namespace from the
    /// Route.
    ///
    /// There are two kinds of parent resources with "Core" support:
    ///
    /// * Gateway (Gateway conformance profile)
    /// * Service (Mesh conformance profile, ClusterIP Services only)
    ///
    /// This API may be extended in the future to support additional kinds of parent
    /// resources.
    ///
    /// ParentRefs must be _distinct_. This means either that:
    ///
    /// * They select different objects.  If this is the case, then parentRef
    ///   entries are distinct. In terms of fields, this means that the
    ///   multi-part key defined by `group`, `kind`, `namespace`, and `name` must
    ///   be unique across all parentRef entries in the Route.
    /// * They do not select different objects, but for each optional field used,
    ///   each ParentRef that selects the same object must set the same set of
    ///   optional fields to different values. If one ParentRef sets a
    ///   combination of optional fields, all must set the same combination.
    ///
    /// Some examples:
    ///
    /// * If one ParentRef sets `sectionName`, all ParentRefs referencing the
    ///   same object must also set `sectionName`.
    /// * If one ParentRef sets `port`, all ParentRefs referencing the same
    ///   object must also set `port`.
    /// * If one ParentRef sets `sectionName` and `port`, all ParentRefs
    ///   referencing the same object must also set `sectionName` and `port`.
    ///
    /// It is possible to separately reference multiple distinct objects that may
    /// be collapsed by an implementation. For example, some implementations may
    /// choose to merge compatible Gateway Listeners together. If that is the
    /// case, the list of routes attached to those resources should also be
    /// merged.
    ///
    /// Note that for ParentRefs that cross namespace boundaries, there are specific
    /// rules. Cross-namespace references are only valid if they are explicitly
    /// allowed by something in the namespace they are referring to. For example,
    /// Gateway has the AllowedRoutes field, and ReferenceGrant provides a
    /// generic way to enable other kinds of cross-namespace reference.
    ///
    ///
    /// ParentRefs from a Route to a Service in the same namespace are "producer"
    /// routes, which apply default routing rules to inbound connections from
    /// any namespace to the Service.
    ///
    /// ParentRefs from a Route to a Service in a different namespace are
    /// "consumer" routes, and these routing rules are only applied to outbound
    /// connections originating from the same namespace as the Route, for which
    /// the intended destination of the connections are a Service targeted as a
    /// ParentRef of the Route.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "parentRefs"
    )]
    pub parent_refs: Option<Vec<HttpRouteParentRefs>>,
    /// Rules are a list of TCP matchers and actions.
    pub rules: Vec<TcpRouteRules>,
    /// UseDefaultGateways indicates the default Gateway scope to use for this
    /// Route. If unset (the default) or set to None, the Route will not be
    /// attached to any default Gateway; if set, it will be attached to any
    /// default Gateway supporting the named scope, subject to the usual rules
    /// about which Routes a Gateway is allowed to claim.
    ///
    /// Think carefully before using this functionality! The set of default
    /// Gateways supporting the requested scope can change over time without
    /// any notice to the Route author, and in many situations it will not be
    /// appropriate to request a default Gateway for a given Route -- for
    /// example, a Route with specific security requirements should almost
    /// certainly not use a default Gateway.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "useDefaultGateways"
    )]
    pub use_default_gateways: Option<GatewayDefaultScope>,
}
/// TCPRouteRule is the configuration for a given rule.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct TcpRouteRules {
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent. If unspecified or invalid (refers to a nonexistent resource or a
    /// Service with no endpoints), the underlying implementation MUST actively
    /// reject connection attempts to this backend. Connection rejections must
    /// respect weight; if an invalid backend is requested to have 80% of
    /// connections, then 80% of connections must be rejected instead.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Extended for Kubernetes ServiceImport
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Extended
    #[serde(rename = "backendRefs")]
    pub backend_refs: Vec<TcpRouteRulesBackendRefs>,
    /// Name is the name of the route rule. This name MUST be unique within a Route if it is set.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// Status defines the current state of TCPRoute.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct TcpRouteStatus {
    /// Parents is a list of parent resources (usually Gateways) that are
    /// associated with the route, and the status of the route with respect to
    /// each parent. When this route attaches to a parent, the controller that
    /// manages the parent must add an entry to this list when the controller
    /// first sees the route and should update the entry as appropriate when the
    /// route or gateway is modified.
    ///
    /// Note that parent references that cannot be resolved by an implementation
    /// of this API will not be added to this list. Implementations of this API
    /// can only populate Route status for the Gateways/parent resources they are
    /// responsible for.
    ///
    /// A maximum of 32 Gateways will be represented in this list. An empty list
    /// means the route has not been attached to any Gateway.
    pub parents: Vec<TcpRouteStatusParents>,
}
/// RouteParentStatus describes the status of a route with respect to an
/// associated Parent.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct TcpRouteStatusParents {
    /// Conditions describes the status of the route with respect to the Gateway.
    /// Note that the route's availability is also subject to the Gateway's own
    /// status conditions and listener status.
    ///
    /// If the Route's ParentRef specifies an existing Gateway that supports
    /// Routes of this kind AND that Gateway's controller has sufficient access,
    /// then that Gateway's controller MUST set the "Accepted" condition on the
    /// Route, to indicate whether the route has been accepted or rejected by the
    /// Gateway, and why.
    ///
    /// A Route MUST be considered "Accepted" if at least one of the Route's
    /// rules is implemented by the Gateway.
    ///
    /// There are a number of cases where the "Accepted" condition may not be set
    /// due to lack of controller visibility, that includes when:
    ///
    /// * The Route refers to a nonexistent parent.
    /// * The Route is of a type that the controller does not support.
    /// * The Route is in a namespace the controller does not have access to.
    pub conditions: Vec<Condition>,
    /// ControllerName is a domain/path string that indicates the name of the
    /// controller that wrote this status. This corresponds with the
    /// controllerName field on GatewayClass.
    ///
    /// Example: "example.net/gateway-controller".
    ///
    /// The format of this field is DOMAIN "/" PATH, where DOMAIN and PATH are
    /// valid Kubernetes names
    /// (<https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names).>
    ///
    /// Controllers MUST populate this field when writing status. Controllers should ensure that
    /// entries to status populated with their ControllerName are cleaned up when they are no
    /// longer necessary.
    #[serde(rename = "controllerName")]
    pub controller_name: String,
    /// ParentRef corresponds with a ParentRef in the spec that this
    /// RouteParentStatus struct describes the status of.
    #[serde(rename = "parentRef")]
    pub parent_ref: HttpRouteParentRefs,
}
