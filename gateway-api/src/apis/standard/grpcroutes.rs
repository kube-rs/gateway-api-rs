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
/// Spec defines the desired state of GRPCRoute.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "GRPCRoute",
    plural = "grpcroutes"
)]
#[kube(namespaced)]
#[kube(status = "RouteStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct GrpcRouteSpec {
    /// Hostnames defines a set of hostnames to match against the GRPC
    /// Host header to select a GRPCRoute to process the request. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label MUST appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and GRPCRoute, there
    /// MUST be at least one intersecting hostname for the GRPCRoute to be
    /// attached to the Listener. For example:
    ///
    /// * A Listener with `test.example.com` as the hostname matches GRPCRoutes
    ///   that have either not specified any hostnames, or have specified at
    ///   least one of `test.example.com` or `*.example.com`.
    /// * A Listener with `*.example.com` as the hostname matches GRPCRoutes
    ///   that have either not specified any hostnames or have specified at least
    ///   one hostname that matches the Listener hostname. For example,
    ///   `test.example.com` and `*.example.com` would both match. On the other
    ///   hand, `example.com` and `test.example.net` would not match.
    ///
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    ///
    /// If both the Listener and GRPCRoute have specified hostnames, any
    /// GRPCRoute hostnames that do not match the Listener hostname MUST be
    /// ignored. For example, if a Listener specified `*.example.com`, and the
    /// GRPCRoute specified `test.example.com` and `test.example.net`,
    /// `test.example.net` MUST NOT be considered for a match.
    ///
    /// If both the Listener and GRPCRoute have specified hostnames, and none
    /// match with the criteria above, then the GRPCRoute MUST NOT be accepted by
    /// the implementation. The implementation MUST raise an 'Accepted' Condition
    /// with a status of `False` in the corresponding RouteParentStatus.
    ///
    /// If a Route (A) of type HTTPRoute or GRPCRoute is attached to a
    /// Listener and that listener already has another Route (B) of the other
    /// type attached and the intersection of the hostnames of A and B is
    /// non-empty, then the implementation MUST accept exactly one of these two
    /// routes, determined by the following criteria, in order:
    ///
    /// * The oldest Route based on creation timestamp.
    /// * The Route appearing first in alphabetical order by
    ///   "{namespace}/{name}".
    ///
    /// The rejected Route MUST raise an 'Accepted' condition with a status of
    /// 'False' in the corresponding RouteParentStatus.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,
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
    /// Rules are a list of GRPC matchers, filters and actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<GrpcRouteRule>>,
}
/// GRPCRouteRule defines the semantics for matching a gRPC request based on
/// conditions (matches), processing it (filters), and forwarding the request to
/// an API object (backendRefs).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteRule {
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent.
    ///
    /// Failure behavior here depends on how many BackendRefs are specified and
    /// how many are invalid.
    ///
    /// If *all* entries in BackendRefs are invalid, and there are also no filters
    /// specified in this route rule, *all* traffic which matches this rule MUST
    /// receive an `UNAVAILABLE` status.
    ///
    /// See the GRPCBackendRef definition for the rules about what makes a single
    /// GRPCBackendRef invalid.
    ///
    /// When a GRPCBackendRef is invalid, `UNAVAILABLE` statuses MUST be returned for
    /// requests that would have otherwise been routed to an invalid backend. If
    /// multiple backends are specified, and some are invalid, the proportion of
    /// requests that would otherwise have been routed to an invalid backend
    /// MUST receive an `UNAVAILABLE` status.
    ///
    /// For example, if two backends are specified with equal weights, and one is
    /// invalid, 50 percent of traffic MUST receive an `UNAVAILABLE` status.
    /// Implementations may choose how that 50 percent is determined.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Core
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backendRefs"
    )]
    pub backend_refs: Option<Vec<GRPCBackendReference>>,
    /// Filters define the filters that are applied to requests that match
    /// this rule.
    ///
    /// The effects of ordering of multiple behaviors are currently unspecified.
    /// This can change in the future based on feedback during the alpha stage.
    ///
    /// Conformance-levels at this level are defined based on the type of filter:
    ///
    /// - ALL core filters MUST be supported by all implementations that support
    ///   GRPCRoute.
    /// - Implementers are encouraged to support extended filters.
    /// - Implementation-specific custom filters have no API guarantees across
    ///   implementations.
    ///
    /// Specifying the same filter multiple times is not supported unless explicitly
    /// indicated in the filter.
    ///
    /// If an implementation cannot support a combination of filters, it must clearly
    /// document that limitation. In cases where incompatible or unsupported
    /// filters are specified and cause the `Accepted` condition to be set to status
    /// `False`, implementations may use the `IncompatibleFilters` reason to specify
    /// this configuration error.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GrpcRouteRulesFilters>>,
    /// Matches define conditions used for matching the rule against incoming
    /// gRPC requests. Each match is independent, i.e. this rule will be matched
    /// if **any** one of the matches is satisfied.
    ///
    /// For example, take the following matches configuration:
    ///
    /// ```text
    /// matches:
    /// - method:
    ///     service: foo.bar
    ///   headers:
    ///     values:
    ///       version: 2
    /// - method:
    ///     service: foo.bar.v2
    /// ```
    ///
    /// For a request to match against this rule, it MUST satisfy
    /// EITHER of the two conditions:
    ///
    /// - service of foo.bar AND contains the header `version: 2`
    /// - service of foo.bar.v2
    ///
    /// See the documentation for GRPCRouteMatch on how to specify multiple
    /// match conditions to be ANDed together.
    ///
    /// If no matches are specified, the implementation MUST match every gRPC request.
    ///
    /// Proxy or Load Balancer routing configuration generated from GRPCRoutes
    /// MUST prioritize rules based on the following criteria, continuing on
    /// ties. Merging MUST not be done between GRPCRoutes and HTTPRoutes.
    /// Precedence MUST be given to the rule with the largest number of:
    ///
    /// * Characters in a matching non-wildcard hostname.
    /// * Characters in a matching hostname.
    /// * Characters in a matching service.
    /// * Characters in a matching method.
    /// * Header matches.
    ///
    /// If ties still exist across multiple Routes, matching precedence MUST be
    /// determined in order of the following criteria, continuing on ties:
    ///
    /// * The oldest Route based on creation timestamp.
    /// * The Route appearing first in alphabetical order by
    ///   "{namespace}/{name}".
    ///
    /// If ties still exist within the Route that has been given precedence,
    /// matching precedence MUST be granted to the first matching rule meeting
    /// the above criteria.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<GrpcRouteMatch>>,
    /// Name is the name of the route rule. This name MUST be unique within a Route if it is set.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// GRPCBackendRef defines how a GRPCRoute forwards a gRPC request.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GRPCBackendReference {
    /// Filters defined at this level MUST be executed if and only if the
    /// request is being forwarded to the backend defined here.
    ///
    /// Support: Implementation-specific (For broader support of filters, use the
    /// Filters field in GRPCRouteRule.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GrpcRouteRulesBackendRefsFilters>>,
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
/// GRPCRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. GRPCRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteRulesBackendRefsFilters {
    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// Support: Implementation-specific
    ///
    /// This filter can be used multiple times within the same rule.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "extensionRef"
    )]
    pub extension_ref: Option<GatewayInfrastructureParametersReference>,
    /// RequestHeaderModifier defines a schema for a filter that modifies request
    /// headers.
    ///
    /// Support: Core
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "requestHeaderModifier"
    )]
    pub request_header_modifier: Option<HeaderModifier>,
    /// RequestMirror defines a schema for a filter that mirrors requests.
    /// Requests are sent to the specified destination, but responses from
    /// that destination are ignored.
    ///
    /// This filter can be used multiple times within the same rule. Note that
    /// not all implementations will be able to support mirroring to multiple
    /// backends.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "requestMirror"
    )]
    pub request_mirror: Option<GrpcRouteRulesBackendRefsFiltersRequestMirror>,
    /// ResponseHeaderModifier defines a schema for a filter that modifies response
    /// headers.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseHeaderModifier"
    )]
    pub response_header_modifier: Option<HeaderModifier>,
    /// Type identifies the type of filter to apply. As with other API fields,
    /// types are classified into three conformance levels:
    ///
    /// - Core: Filter types and their corresponding configuration defined by
    ///   "Support: Core" in this package, e.g. "RequestHeaderModifier". All
    ///   implementations supporting GRPCRoute MUST support core filters.
    ///
    /// - Extended: Filter types and their corresponding configuration defined by
    ///   "Support: Extended" in this package, e.g. "RequestMirror". Implementers
    ///   are encouraged to support extended filters.
    ///
    /// - Implementation-specific: Filters that are defined and supported by specific vendors.
    ///   In the future, filters showing convergence in behavior across multiple
    ///   implementations will be considered for inclusion in extended or core
    ///   conformance levels. Filter-specific configuration for such filters
    ///   is specified using the ExtensionRef field. `Type` MUST be set to
    ///   "ExtensionRef" for custom filters.
    ///
    /// Implementers are encouraged to define custom implementation types to
    /// extend the core API with implementation-specific behavior.
    ///
    /// If a reference to a custom filter type cannot be resolved, the filter
    /// MUST NOT be skipped. Instead, requests that would have been processed by
    /// that filter MUST receive a HTTP error response.
    #[serde(rename = "type")]
    pub r#type: GRPCFilterType,
}
/// RequestMirror defines a schema for a filter that mirrors requests.
/// Requests are sent to the specified destination, but responses from
/// that destination are ignored.
///
/// This filter can be used multiple times within the same rule. Note that
/// not all implementations will be able to support mirroring to multiple
/// backends.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteRulesBackendRefsFiltersRequestMirror {
    /// BackendRef references a resource where mirrored requests are sent.
    ///
    /// Mirrored requests must be sent only to a single destination endpoint
    /// within this BackendRef, irrespective of how many endpoints are present
    /// within this BackendRef.
    ///
    /// If the referent cannot be found, this BackendRef is invalid and must be
    /// dropped from the Gateway. The controller must ensure the "ResolvedRefs"
    /// condition on the Route status is set to `status: False` and not configure
    /// this backend in the underlying implementation.
    ///
    /// If there is a cross-namespace reference to an *existing* object
    /// that is not allowed by a ReferenceGrant, the controller must ensure the
    /// "ResolvedRefs"  condition on the Route is set to `status: False`,
    /// with the "RefNotPermitted" reason and not configure this backend in the
    /// underlying implementation.
    ///
    /// In either error case, the Message of the `ResolvedRefs` Condition
    /// should be used to provide more detail about the problem.
    ///
    /// Support: Extended for Kubernetes Service
    ///
    /// Support: Implementation-specific for any other resource
    #[serde(rename = "backendRef")]
    pub backend_ref: RequestMirrorReference,
    /// Fraction represents the fraction of requests that should be
    /// mirrored to BackendRef.
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fraction: Option<HttpRouteRulesBackendRefsFiltersRequestMirrorFraction>,
    /// Percent represents the percentage of requests that should be
    /// mirrored to BackendRef. Its minimum value is 0 (indicating 0% of
    /// requests) and its maximum value is 100 (indicating 100% of requests).
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
}
/// GRPCRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. GRPCRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteRulesFilters {
    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// Support: Implementation-specific
    ///
    /// This filter can be used multiple times within the same rule.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "extensionRef"
    )]
    pub extension_ref: Option<GatewayInfrastructureParametersReference>,
    /// RequestHeaderModifier defines a schema for a filter that modifies request
    /// headers.
    ///
    /// Support: Core
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "requestHeaderModifier"
    )]
    pub request_header_modifier: Option<HeaderModifier>,
    /// RequestMirror defines a schema for a filter that mirrors requests.
    /// Requests are sent to the specified destination, but responses from
    /// that destination are ignored.
    ///
    /// This filter can be used multiple times within the same rule. Note that
    /// not all implementations will be able to support mirroring to multiple
    /// backends.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "requestMirror"
    )]
    pub request_mirror: Option<GrpcRouteRulesFiltersRequestMirror>,
    /// ResponseHeaderModifier defines a schema for a filter that modifies response
    /// headers.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseHeaderModifier"
    )]
    pub response_header_modifier: Option<HeaderModifier>,
    /// Type identifies the type of filter to apply. As with other API fields,
    /// types are classified into three conformance levels:
    ///
    /// - Core: Filter types and their corresponding configuration defined by
    ///   "Support: Core" in this package, e.g. "RequestHeaderModifier". All
    ///   implementations supporting GRPCRoute MUST support core filters.
    ///
    /// - Extended: Filter types and their corresponding configuration defined by
    ///   "Support: Extended" in this package, e.g. "RequestMirror". Implementers
    ///   are encouraged to support extended filters.
    ///
    /// - Implementation-specific: Filters that are defined and supported by specific vendors.
    ///   In the future, filters showing convergence in behavior across multiple
    ///   implementations will be considered for inclusion in extended or core
    ///   conformance levels. Filter-specific configuration for such filters
    ///   is specified using the ExtensionRef field. `Type` MUST be set to
    ///   "ExtensionRef" for custom filters.
    ///
    /// Implementers are encouraged to define custom implementation types to
    /// extend the core API with implementation-specific behavior.
    ///
    /// If a reference to a custom filter type cannot be resolved, the filter
    /// MUST NOT be skipped. Instead, requests that would have been processed by
    /// that filter MUST receive a HTTP error response.
    #[serde(rename = "type")]
    pub r#type: GRPCFilterType,
}
/// RequestMirror defines a schema for a filter that mirrors requests.
/// Requests are sent to the specified destination, but responses from
/// that destination are ignored.
///
/// This filter can be used multiple times within the same rule. Note that
/// not all implementations will be able to support mirroring to multiple
/// backends.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteRulesFiltersRequestMirror {
    /// BackendRef references a resource where mirrored requests are sent.
    ///
    /// Mirrored requests must be sent only to a single destination endpoint
    /// within this BackendRef, irrespective of how many endpoints are present
    /// within this BackendRef.
    ///
    /// If the referent cannot be found, this BackendRef is invalid and must be
    /// dropped from the Gateway. The controller must ensure the "ResolvedRefs"
    /// condition on the Route status is set to `status: False` and not configure
    /// this backend in the underlying implementation.
    ///
    /// If there is a cross-namespace reference to an *existing* object
    /// that is not allowed by a ReferenceGrant, the controller must ensure the
    /// "ResolvedRefs"  condition on the Route is set to `status: False`,
    /// with the "RefNotPermitted" reason and not configure this backend in the
    /// underlying implementation.
    ///
    /// In either error case, the Message of the `ResolvedRefs` Condition
    /// should be used to provide more detail about the problem.
    ///
    /// Support: Extended for Kubernetes Service
    ///
    /// Support: Implementation-specific for any other resource
    #[serde(rename = "backendRef")]
    pub backend_ref: RequestMirrorReference,
    /// Fraction represents the fraction of requests that should be
    /// mirrored to BackendRef.
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fraction: Option<HttpRouteRulesBackendRefsFiltersRequestMirrorFraction>,
    /// Percent represents the percentage of requests that should be
    /// mirrored to BackendRef. Its minimum value is 0 (indicating 0% of
    /// requests) and its maximum value is 100 (indicating 100% of requests).
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
}
/// GRPCRouteMatch defines the predicate used to match requests to a given
/// action. Multiple match types are ANDed together, i.e. the match will
/// evaluate to true only if all conditions are satisfied.
///
/// For example, the match below will match a gRPC request only if its service
/// is `foo` AND it contains the `version: v1` header:
///
/// ```text
/// matches:
///   - method:
///     type: Exact
///     service: "foo"
///     headers:
///   - name: "version"
///     value "v1"
///
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteMatch {
    /// Headers specifies gRPC request header matchers. Multiple match values are
    /// ANDed together, meaning, a request MUST match all the specified headers
    /// to select the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HeaderMatch>>,
    /// Method specifies a gRPC request service/method matcher. If this field is
    /// not specified, all services and methods will match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<GRPCMethodMatch>,
}
/// Method specifies a gRPC request service/method matcher. If this field is
/// not specified, all services and methods will match.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GRPCMethodMatch {
    /// Value of the method to match against. If left empty or omitted, will
    /// match all services.
    ///
    /// At least one of Service and Method MUST be a non-empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Value of the service to match against. If left empty or omitted, will
    /// match any service.
    ///
    /// At least one of Service and Method MUST be a non-empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Type specifies how to match against the service and/or method.
    /// Support: Core (Exact with service and method specified)
    ///
    /// Support: Implementation-specific (Exact with method specified but no service specified)
    ///
    /// Support: Implementation-specific (RegularExpression)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<HeaderMatchType>,
}
