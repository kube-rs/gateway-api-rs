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
    version = "v1alpha3",
    kind = "TLSRoute",
    plural = "tlsroutes"
)]
#[kube(namespaced)]
#[kube(status = "RouteStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct TLSRouteSpec {
    /// Hostnames defines a set of SNI hostnames that should match against the
    /// SNI attribute of TLS ClientHello message in TLS handshake. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed in SNI hostnames per RFC 6066.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label must appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and TLSRoute, there
    /// must be at least one intersecting hostname for the TLSRoute to be
    /// attached to the Listener. For example:
    ///
    /// * A Listener with `test.example.com` as the hostname matches TLSRoutes
    ///   that have specified at least one of `test.example.com` or
    ///   `*.example.com`.
    /// * A Listener with `*.example.com` as the hostname matches TLSRoutes
    ///   that have specified at least one hostname that matches the Listener
    ///   hostname. For example, `test.example.com` and `*.example.com` would both
    ///   match. On the other hand, `example.com` and `test.example.net` would not
    ///   match.
    ///
    /// If both the Listener and TLSRoute have specified hostnames, any
    /// TLSRoute hostnames that do not match the Listener hostname MUST be
    /// ignored. For example, if a Listener specified `*.example.com`, and the
    /// TLSRoute specified `test.example.com` and `test.example.net`,
    /// `test.example.net` must not be considered for a match.
    ///
    /// If both the Listener and TLSRoute have specified hostnames, and none
    /// match with the criteria above, then the TLSRoute is not accepted. The
    /// implementation must raise an 'Accepted' Condition with a status of
    /// `False` in the corresponding RouteParentStatus.
    ///
    /// Support: Core
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
    pub parent_refs: Option<Vec<ParentReference>>,
    /// Rules are a list of actions.
    pub rules: Vec<CommonRouteRule>,
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
