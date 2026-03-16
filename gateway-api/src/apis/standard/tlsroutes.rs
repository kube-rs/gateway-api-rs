// WARNING: generated file - manual changes will be overriden

use super::common::*;
#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube_derive::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}
use self::prelude::*;
/// Spec defines the desired state of TLSRoute.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "TLSRoute",
    plural = "tlsroutes"
)]
#[kube(namespaced)]
#[kube(status = "RouteStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct TlsRouteSpec {
    /// Hostnames defines a set of SNI hostnames that should match against the
    /// SNI attribute of TLS ClientHello message in TLS handshake. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed in SNI hostnames per RFC 6066.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label must appear by itself as the first label.
    pub hostnames: Vec<String>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "parentRefs"
    )]
    pub parent_refs: Option<Vec<ParentReference>>,
    /// Rules are a list of actions.
    pub rules: Vec<TlsRouteRules>,
}
/// TLSRouteRule is the configuration for a given rule.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct TlsRouteRules {
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent. If unspecified or invalid (refers to a nonexistent resource or
    /// a Service with no endpoints), the rule performs no forwarding; if no
    /// filters are specified that would result in a response being sent, the
    /// underlying implementation must actively reject request attempts to this
    /// backend, by rejecting the connection. Request rejections must respect
    /// weight; if an invalid backend is requested to have 80% of requests, then
    /// 80% of requests must be rejected instead.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Extended for Kubernetes ServiceImport
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Extended
    #[serde(rename = "backendRefs")]
    pub backend_refs: Vec<TlsRouteRulesBackendRefs>,
    /// Name is the name of the route rule. This name MUST be unique within a Route if it is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// BackendRef defines how a Route should forward a request to a Kubernetes
/// resource.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
///
/// Note that when the BackendTLSPolicy object is enabled by the implementation,
/// there are some extra rules about validity to consider here. See the fields
/// where this struct is used for more information about the exact behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct TlsRouteRulesBackendRefs {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is the Kubernetes resource kind of the referent. For example
    /// "Service".
    ///
    /// Defaults to "Service" when not specified.
    ///
    /// ExternalName services can refer to CNAME DNS records that may live
    /// outside of the cluster and as such are difficult to reason about in
    /// terms of conformance. They also may not be safe to forward to (see
    /// CVE-2021-25740 for more information). Implementations SHOULD NOT
    /// support ExternalName Services.
    ///
    /// Support: Core (Services with a type other than ExternalName)
    ///
    /// Support: Implementation-specific (Services with type ExternalName)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the backend. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Weight specifies the proportion of requests forwarded to the referenced
    /// backend. This is computed as weight/(sum of all weights in this
    /// BackendRefs list). For non-zero values, there may be some epsilon from
    /// the exact proportion defined here depending on the precision an
    /// implementation supports. Weight is not a percentage and the sum of
    /// weights does not need to equal 100.
    ///
    /// If only one backend is specified and it has a weight greater than 0, 100%
    /// of the traffic is forwarded to that backend. If weight is set to 0, no
    /// traffic should be forwarded for this entry. If unspecified, weight
    /// defaults to 1.
    ///
    /// Support for this field varies based on the context where used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
