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
/// Spec defines the desired state of ListenerSet.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "ListenerSet",
    plural = "listenersets"
)]
#[kube(namespaced)]
#[kube(status = "ListenerSetStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct ListenerSetSpec {
    /// Listeners associated with this ListenerSet. Listeners define
    /// logical endpoints that are bound on this referenced parent Gateway's addresses.
    ///
    /// Listeners in a `Gateway` and their attached `ListenerSets` are concatenated
    /// as a list when programming the underlying infrastructure. Each listener
    /// name does not need to be unique across the Gateway and ListenerSets.
    /// See ListenerEntry.Name for more details.
    ///
    /// Implementations MUST treat the parent Gateway as having the merged
    /// list of all listeners from itself and attached ListenerSets using
    /// the following precedence:
    ///
    /// 1. "parent" Gateway
    /// 2. ListenerSet ordered by creation time (oldest first)
    /// 3. ListenerSet ordered alphabetically by "{namespace}/{name}".
    ///
    /// An implementation MAY reject listeners by setting the ListenerEntryStatus
    /// `Accepted` condition to False with the Reason `TooManyListeners`
    ///
    /// If a listener has a conflict, this will be reported in the
    /// Status.ListenerEntryStatus setting the `Conflicted` condition to True.
    ///
    /// Implementations SHOULD be cautious about what information from the
    /// parent or siblings are reported to avoid accidentally leaking
    /// sensitive information that the child would not otherwise have access
    /// to. This can include contents of secrets etc.
    pub listeners: Vec<ListenerSetListeners>,
    /// ParentRef references the Gateway that the listeners are attached to.
    #[serde(rename = "parentRef")]
    pub parent_ref: Reference,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerSetListeners {
    /// AllowedRoutes defines the types of routes that MAY be attached to a
    /// Listener and the trusted namespaces where those Route resources MAY be
    /// present.
    ///
    /// Although a client request may match multiple route rules, only one rule
    /// may ultimately receive the request. Matching precedence MUST be
    /// determined in order of the following criteria:
    ///
    /// * The most specific match as defined by the Route type.
    /// * The oldest Route based on creation timestamp. For example, a Route with
    ///   a creation timestamp of "2020-09-08 01:02:03" is given precedence over
    ///   a Route with a creation timestamp of "2020-09-08 01:02:04".
    /// * If everything else is equivalent, the Route appearing first in
    ///   alphabetical order (namespace/name) should be given precedence. For
    ///   example, foo/bar is given precedence over foo/baz.
    ///
    /// All valid rules within a Route attached to this Listener should be
    /// implemented. Invalid Route rules can be ignored (sometimes that will mean
    /// the full Route). If a Route rule transitions from valid to invalid,
    /// support for that Route rule should be dropped to ensure consistency. For
    /// example, even if a filter specified by a Route rule is invalid, the rest
    /// of the rules within that Route should still be supported.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedRoutes"
    )]
    pub allowed_routes: Option<ListenerSetListenersAllowedRoutes>,
    /// Hostname specifies the virtual hostname to match for protocol types that
    /// define this concept. When unspecified, all hostnames are matched. This
    /// field is ignored for protocols that don't require hostname based
    /// matching.
    ///
    /// Implementations MUST apply Hostname matching appropriately for each of
    /// the following protocols:
    ///
    /// * TLS: The Listener Hostname MUST match the SNI.
    /// * HTTP: The Listener Hostname MUST match the Host header of the request.
    /// * HTTPS: The Listener Hostname SHOULD match at both the TLS and HTTP
    ///   protocol layers as described above. If an implementation does not
    ///   ensure that both the SNI and Host header match the Listener hostname,
    ///   it MUST clearly document that.
    ///
    /// For HTTPRoute and TLSRoute resources, there is an interaction with the
    /// `spec.hostnames` array. When both listener and route specify hostnames,
    /// there MUST be an intersection between the values for a Route to be
    /// accepted. For more information, refer to the Route specific Hostnames
    /// documentation.
    ///
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Name is the name of the Listener. This name MUST be unique within a
    /// ListenerSet.
    ///
    /// Name is not required to be unique across a Gateway and ListenerSets.
    /// Routes can attach to a Listener by having a ListenerSet as a parentRef
    /// and setting the SectionName
    pub name: String,
    /// Port is the network port. Multiple listeners may use the
    /// same port, subject to the Listener compatibility rules.
    pub port: i32,
    /// Protocol specifies the network protocol this listener expects to receive.
    pub protocol: String,
    /// TLS is the TLS configuration for the Listener. This field is required if
    /// the Protocol field is "HTTPS" or "TLS". It is invalid to set this field
    /// if the Protocol field is "HTTP", "TCP", or "UDP".
    ///
    /// The association of SNIs to Certificate defined in ListenerTLSConfig is
    /// defined based on the Hostname field for this listener.
    ///
    /// The GatewayClass MUST use the longest matching SNI out of all
    /// available certificates for any TLS handshake.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<GatewayListenersTls>,
}
/// AllowedRoutes defines the types of routes that MAY be attached to a
/// Listener and the trusted namespaces where those Route resources MAY be
/// present.
///
/// Although a client request may match multiple route rules, only one rule
/// may ultimately receive the request. Matching precedence MUST be
/// determined in order of the following criteria:
///
/// * The most specific match as defined by the Route type.
/// * The oldest Route based on creation timestamp. For example, a Route with
///   a creation timestamp of "2020-09-08 01:02:03" is given precedence over
///   a Route with a creation timestamp of "2020-09-08 01:02:04".
/// * If everything else is equivalent, the Route appearing first in
///   alphabetical order (namespace/name) should be given precedence. For
///   example, foo/bar is given precedence over foo/baz.
///
/// All valid rules within a Route attached to this Listener should be
/// implemented. Invalid Route rules can be ignored (sometimes that will mean
/// the full Route). If a Route rule transitions from valid to invalid,
/// support for that Route rule should be dropped to ensure consistency. For
/// example, even if a filter specified by a Route rule is invalid, the rest
/// of the rules within that Route should still be supported.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerSetListenersAllowedRoutes {
    /// Kinds specifies the groups and kinds of Routes that are allowed to bind
    /// to this Gateway Listener. When unspecified or empty, the kinds of Routes
    /// selected are determined using the Listener protocol.
    ///
    /// A RouteGroupKind MUST correspond to kinds of Routes that are compatible
    /// with the application protocol specified in the Listener's Protocol field.
    /// If an implementation does not support or recognize this resource type, it
    /// MUST set the "ResolvedRefs" condition to False for this Listener with the
    /// "InvalidRouteKinds" reason.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<Kind>>,
    /// Namespaces indicates namespaces from which Routes may be attached to this
    /// Listener. This is restricted to the namespace of this Gateway by default.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<ListenerSetListenersAllowedRoutesNamespaces>,
}
/// Namespaces indicates namespaces from which Routes may be attached to this
/// Listener. This is restricted to the namespace of this Gateway by default.
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerSetListenersAllowedRoutesNamespaces {
    /// From indicates where Routes will be selected for this Gateway. Possible
    /// values are:
    ///
    /// * All: Routes in all namespaces may be used by this Gateway.
    /// * Selector: Routes in namespaces selected by the selector may be used by
    ///   this Gateway.
    /// * Same: Only Routes in the same namespace may be used by this Gateway.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<AllowedRoutesNamespaces>,
    /// Selector must be specified when From is set to "Selector". In that case,
    /// only Routes in Namespaces matching this Selector will be selected by this
    /// Gateway. This field is ignored for other values of "From".
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<GatewayAllowedListenersNamespacesSelector>,
}
/// Status defines the current state of ListenerSet.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerSetStatus {
    /// Conditions describe the current conditions of the ListenerSet.
    ///
    /// Implementations MUST express ListenerSet conditions using the
    /// `ListenerSetConditionType` and `ListenerSetConditionReason`
    /// constants so that operators and tools can converge on a common
    /// vocabulary to describe ListenerSet state.
    ///
    /// Known condition types are:
    ///
    /// * "Accepted"
    /// * "Programmed"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Listeners provide status for each unique listener port defined in the Spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<GatewayStatusListeners>>,
}
