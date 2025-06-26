// WARNING! generated file do not edit

use super::common::*;
#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
}
use self::prelude::*;
/// Spec defines the desired state of HTTPRoute.
#[derive(
    CustomResource,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    JsonSchema,
    Default,
    PartialEq
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "HTTPRoute",
    plural = "httproutes"
)]
#[kube(namespaced)]
#[kube(status = "RouteStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct HTTPRouteSpec {
    /// Hostnames defines a set of hostnames that should match against the HTTP Host
    /// header to select a HTTPRoute used to process the request. Implementations
    /// MUST ignore any port value specified in the HTTP Host header while
    /// performing a match and (absent of any applicable header modification
    /// configuration) MUST forward this header unmodified to the backend.
    ///
    /// Valid values for Hostnames are determined by RFC 1123 definition of a
    /// hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label must appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and HTTPRoute, there
    /// must be at least one intersecting hostname for the HTTPRoute to be
    /// attached to the Listener. For example:
    ///
    /// * A Listener with `test.example.com` as the hostname matches HTTPRoutes
    ///   that have either not specified any hostnames, or have specified at
    ///   least one of `test.example.com` or `*.example.com`.
    /// * A Listener with `*.example.com` as the hostname matches HTTPRoutes
    ///   that have either not specified any hostnames or have specified at least
    ///   one hostname that matches the Listener hostname. For example,
    ///   `*.example.com`, `test.example.com`, and `foo.test.example.com` would
    ///   all match. On the other hand, `example.com` and `test.example.net` would
    ///   not match.
    ///
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    ///
    /// If both the Listener and HTTPRoute have specified hostnames, any
    /// HTTPRoute hostnames that do not match the Listener hostname MUST be
    /// ignored. For example, if a Listener specified `*.example.com`, and the
    /// HTTPRoute specified `test.example.com` and `test.example.net`,
    /// `test.example.net` must not be considered for a match.
    ///
    /// If both the Listener and HTTPRoute have specified hostnames, and none
    /// match with the criteria above, then the HTTPRoute is not accepted. The
    /// implementation must raise an 'Accepted' Condition with a status of
    /// `False` in the corresponding RouteParentStatus.
    ///
    /// In the event that multiple HTTPRoutes specify intersecting hostnames (e.g.
    /// overlapping wildcard matching and exact matching hostnames), precedence must
    /// be given to rules from the HTTPRoute with the largest number of:
    ///
    /// * Characters in a matching non-wildcard hostname.
    /// * Characters in a matching hostname.
    ///
    /// If ties exist across multiple Routes, the matching precedence rules for
    /// HTTPRouteMatches takes over.
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
    ///
    ///
    ///
    ///
    ///
    ///
    ///
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentRefs")]
    pub parent_refs: Option<Vec<ParentReference>>,
    /// Rules are a list of HTTP matchers, filters and actions.
    ///
    ///
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<HTTPRouteRule>>,
}
/// HTTPRouteRule defines semantics for matching an HTTP request based on
/// conditions (matches), processing it (filters), and forwarding the request to
/// an API object (backendRefs).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRule {
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent.
    ///
    /// Failure behavior here depends on how many BackendRefs are specified and
    /// how many are invalid.
    ///
    /// If *all* entries in BackendRefs are invalid, and there are also no filters
    /// specified in this route rule, *all* traffic which matches this rule MUST
    /// receive a 500 status code.
    ///
    /// See the HTTPBackendRef definition for the rules about what makes a single
    /// HTTPBackendRef invalid.
    ///
    /// When a HTTPBackendRef is invalid, 500 status codes MUST be returned for
    /// requests that would have otherwise been routed to an invalid backend. If
    /// multiple backends are specified, and some are invalid, the proportion of
    /// requests that would otherwise have been routed to an invalid backend
    /// MUST receive a 500 status code.
    ///
    /// For example, if two backends are specified with equal weights, and one is
    /// invalid, 50 percent of traffic must receive a 500. Implementations may
    /// choose how that 50 percent is determined.
    ///
    /// When a HTTPBackendRef refers to a Service that has no ready endpoints,
    /// implementations SHOULD return a 503 for requests to that backend instead.
    /// If an implementation chooses to do this, all of the above rules for 500 responses
    /// MUST also apply for responses that return a 503.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Extended for Kubernetes ServiceImport
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendRefs")]
    pub backend_refs: Option<Vec<HTTPBackendReference>>,
    /// Filters define the filters that are applied to requests that match
    /// this rule.
    ///
    /// Wherever possible, implementations SHOULD implement filters in the order
    /// they are specified.
    ///
    /// Implementations MAY choose to implement this ordering strictly, rejecting
    /// any combination or order of filters that can not be supported. If implementations
    /// choose a strict interpretation of filter ordering, they MUST clearly document
    /// that behavior.
    ///
    /// To reject an invalid combination or order of filters, implementations SHOULD
    /// consider the Route Rules with this configuration invalid. If all Route Rules
    /// in a Route are invalid, the entire Route would be considered invalid. If only
    /// a portion of Route Rules are invalid, implementations MUST set the
    /// "PartiallyInvalid" condition for the Route.
    ///
    /// Conformance-levels at this level are defined based on the type of filter:
    ///
    /// - ALL core filters MUST be supported by all implementations.
    /// - Implementers are encouraged to support extended filters.
    /// - Implementation-specific custom filters have no API guarantees across
    ///   implementations.
    ///
    /// Specifying the same filter multiple times is not supported unless explicitly
    /// indicated in the filter.
    ///
    /// All filters are expected to be compatible with each other except for the
    /// URLRewrite and RequestRedirect filters, which may not be combined. If an
    /// implementation can not support other combinations of filters, they must clearly
    /// document that limitation. In cases where incompatible or unsupported
    /// filters are specified and cause the `Accepted` condition to be set to status
    /// `False`, implementations may use the `IncompatibleFilters` reason to specify
    /// this configuration error.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<HTTPRouteFilter>>,
    /// Matches define conditions used for matching the rule against incoming
    /// HTTP requests. Each match is independent, i.e. this rule will be matched
    /// if **any** one of the matches is satisfied.
    ///
    /// For example, take the following matches configuration:
    ///
    /// ```text
    /// matches:
    /// - path:
    ///     value: "/foo"
    ///   headers:
    ///   - name: "version"
    ///     value: "v2"
    /// - path:
    ///     value: "/v2/foo"
    /// ```
    ///
    /// For a request to match against this rule, a request must satisfy
    /// EITHER of the two conditions:
    ///
    /// - path prefixed with `/foo` AND contains the header `version: v2`
    /// - path prefix of `/v2/foo`
    ///
    /// See the documentation for HTTPRouteMatch on how to specify multiple
    /// match conditions that should be ANDed together.
    ///
    /// If no matches are specified, the default is a prefix
    /// path match on "/", which has the effect of matching every
    /// HTTP request.
    ///
    /// Proxy or Load Balancer routing configuration generated from HTTPRoutes
    /// MUST prioritize matches based on the following criteria, continuing on
    /// ties. Across all rules specified on applicable Routes, precedence must be
    /// given to the match having:
    ///
    /// * "Exact" path match.
    /// * "Prefix" path match with largest number of characters.
    /// * Method match.
    /// * Largest number of header matches.
    /// * Largest number of query param matches.
    ///
    /// Note: The precedence of RegularExpression path matches are implementation-specific.
    ///
    /// If ties still exist across multiple Routes, matching precedence MUST be
    /// determined in order of the following criteria, continuing on ties:
    ///
    /// * The oldest Route based on creation timestamp.
    /// * The Route appearing first in alphabetical order by
    ///   "{namespace}/{name}".
    ///
    /// If ties still exist within an HTTPRoute, matching precedence MUST be granted
    /// to the FIRST matching rule (in list order) with a match meeting the above
    /// criteria.
    ///
    /// When no rules matching a request have been successfully attached to the
    /// parent a request is coming from, a HTTP 404 status code MUST be returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<RouteMatch>>,
    /// Timeouts defines the timeouts that can be configured for an HTTP request.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<HTTPRouteTimeout>,
}
/// HTTPBackendRef defines how a HTTPRoute forwards a HTTP request.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
///
/// <gateway:experimental:description>
///
/// When the BackendRef points to a Kubernetes Service, implementations SHOULD
/// honor the appProtocol field if it is set for the target Service Port.
///
/// Implementations supporting appProtocol SHOULD recognize the Kubernetes
/// Standard Application Protocols defined in KEP-3726.
///
/// If a Service appProtocol isn't specified, an implementation MAY infer the
/// backend protocol through its own means. Implementations MAY infer the
/// protocol from the Route type referring to the backend Service.
///
/// If a Route is not able to send traffic to the backend using the specified
/// protocol then the backend is considered invalid. Implementations MUST set the
/// "ResolvedRefs" condition to "False" with the "UnsupportedProtocol" reason.
///
/// </gateway:experimental:description>
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPBackendReference {
    /// Filters defined at this level should be executed if and only if the
    /// request is being forwarded to the backend defined here.
    ///
    /// Support: Implementation-specific (For broader support of filters, use the
    /// Filters field in HTTPRouteRule.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<HTTPRouteBackendFilter>>,
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
/// HTTPRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. HTTPRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteBackendFilter {
    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// This filter can be used multiple times within the same rule.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extensionRef")]
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
    ///
    ///
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestMirror")]
    pub request_mirror: Option<RequestMirror>,
    /// RequestRedirect defines a schema for a filter that responds to the
    /// request with an HTTP redirection.
    ///
    /// Support: Core
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "requestRedirect"
    )]
    pub request_redirect: Option<RequestRedirect>,
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
    ///   implementations must support core filters.
    ///
    /// - Extended: Filter types and their corresponding configuration defined by
    ///   "Support: Extended" in this package, e.g. "RequestMirror". Implementers
    ///   are encouraged to support extended filters.
    ///
    /// - Implementation-specific: Filters that are defined and supported by
    ///   specific vendors.
    ///   In the future, filters showing convergence in behavior across multiple
    ///   implementations will be considered for inclusion in extended or core
    ///   conformance levels. Filter-specific configuration for such filters
    ///   is specified using the ExtensionRef field. `Type` should be set to
    ///   "ExtensionRef" for custom filters.
    ///
    /// Implementers are encouraged to define custom implementation types to
    /// extend the core API with implementation-specific behavior.
    ///
    /// If a reference to a custom filter type cannot be resolved, the filter
    /// MUST NOT be skipped. Instead, requests that would have been processed by
    /// that filter MUST receive a HTTP error response.
    ///
    /// Note that values may be added to this enum, implementations
    /// must ensure that unknown values will not cause a crash.
    ///
    /// Unknown values here must result in the implementation setting the
    /// Accepted Condition for the Route to `status: False`, with a
    /// Reason of `UnsupportedValue`.
    #[serde(rename = "type")]
    pub r#type: HTTPFilterType,
    /// URLRewrite defines a schema for a filter that modifies a request during forwarding.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "urlRewrite")]
    pub url_rewrite: Option<HTTPRouteUrlRewrite>,
}
/// HTTPRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. HTTPRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteFilter {
    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// This filter can be used multiple times within the same rule.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extensionRef")]
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
    ///
    ///
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestMirror")]
    pub request_mirror: Option<RequestMirror>,
    /// RequestRedirect defines a schema for a filter that responds to the
    /// request with an HTTP redirection.
    ///
    /// Support: Core
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "requestRedirect"
    )]
    pub request_redirect: Option<RequestRedirect>,
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
    ///   implementations must support core filters.
    ///
    /// - Extended: Filter types and their corresponding configuration defined by
    ///   "Support: Extended" in this package, e.g. "RequestMirror". Implementers
    ///   are encouraged to support extended filters.
    ///
    /// - Implementation-specific: Filters that are defined and supported by
    ///   specific vendors.
    ///   In the future, filters showing convergence in behavior across multiple
    ///   implementations will be considered for inclusion in extended or core
    ///   conformance levels. Filter-specific configuration for such filters
    ///   is specified using the ExtensionRef field. `Type` should be set to
    ///   "ExtensionRef" for custom filters.
    ///
    /// Implementers are encouraged to define custom implementation types to
    /// extend the core API with implementation-specific behavior.
    ///
    /// If a reference to a custom filter type cannot be resolved, the filter
    /// MUST NOT be skipped. Instead, requests that would have been processed by
    /// that filter MUST receive a HTTP error response.
    ///
    /// Note that values may be added to this enum, implementations
    /// must ensure that unknown values will not cause a crash.
    ///
    /// Unknown values here must result in the implementation setting the
    /// Accepted Condition for the Route to `status: False`, with a
    /// Reason of `UnsupportedValue`.
    #[serde(rename = "type")]
    pub r#type: HTTPFilterType,
    /// URLRewrite defines a schema for a filter that modifies a request during forwarding.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "urlRewrite")]
    pub url_rewrite: Option<HTTPRouteUrlRewrite>,
}
/// HTTPRouteMatch defines the predicate used to match requests to a given
/// action. Multiple match types are ANDed together, i.e. the match will
/// evaluate to true only if all conditions are satisfied.
///
/// For example, the match below will match a HTTP request only if its path
/// starts with `/foo` AND it contains the `version: v1` header:
///
/// ```text
/// match:
///
/// 	path:
/// 	  value: "/foo"
/// 	headers:
/// 	- name: "version"
/// 	  value "v1"
///
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RouteMatch {
    /// Headers specifies HTTP request header matchers. Multiple match values are
    /// ANDed together, meaning, a request must match all the specified headers
    /// to select the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HeaderMatch>>,
    /// Method specifies HTTP method matcher.
    /// When specified, this route will be matched only if the request has the
    /// specified method.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<HTTPMethodMatch>,
    /// Path specifies a HTTP request path matcher. If this field is not
    /// specified, a default prefix match on the "/" path is provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<PathMatch>,
    /// QueryParams specifies HTTP query parameter matchers. Multiple match
    /// values are ANDed together, meaning, a request must match all the
    /// specified query parameters to select the route.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryParams")]
    pub query_params: Option<Vec<HeaderMatch>>,
}
/// HTTPRouteMatch defines the predicate used to match requests to a given
/// action. Multiple match types are ANDed together, i.e. the match will
/// evaluate to true only if all conditions are satisfied.
///
/// For example, the match below will match a HTTP request only if its path
/// starts with `/foo` AND it contains the `version: v1` header:
///
/// ```text
/// match:
///
/// 	path:
/// 	  value: "/foo"
/// 	headers:
/// 	- name: "version"
/// 	  value "v1"
///
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HTTPMethodMatch {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "TRACE")]
    Trace,
    #[serde(rename = "PATCH")]
    Patch,
}
/// Path specifies a HTTP request path matcher. If this field is not
/// specified, a default prefix match on the "/" path is provided.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct PathMatch {
    /// Type specifies how to match against the path Value.
    ///
    /// Support: Core (Exact, PathPrefix)
    ///
    /// Support: Implementation-specific (RegularExpression)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<HTTPRouteRulesMatchesPathType>,
    /// Value of the HTTP path to match against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// Path specifies a HTTP request path matcher. If this field is not
/// specified, a default prefix match on the "/" path is provided.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HTTPRouteRulesMatchesPathType {
    Exact,
    PathPrefix,
    RegularExpression,
}
/// Timeouts defines the timeouts that can be configured for an HTTP request.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteTimeout {
    /// BackendRequest specifies a timeout for an individual request from the gateway
    /// to a backend. This covers the time from when the request first starts being
    /// sent from the gateway to when the full response has been received from the backend.
    ///
    /// Setting a timeout to the zero duration (e.g. "0s") SHOULD disable the timeout
    /// completely. Implementations that cannot completely disable the timeout MUST
    /// instead interpret the zero duration as the longest possible value to which
    /// the timeout can be set.
    ///
    /// An entire client HTTP transaction with a gateway, covered by the Request timeout,
    /// may result in more than one call from the gateway to the destination backend,
    /// for example, if automatic retries are supported.
    ///
    /// The value of BackendRequest must be a Gateway API Duration string as defined by
    /// GEP-2257.  When this field is unspecified, its behavior is implementation-specific;
    /// when specified, the value of BackendRequest must be no more than the value of the
    /// Request timeout (since the Request timeout encompasses the BackendRequest timeout).
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendRequest")]
    pub backend_request: Option<String>,
    /// Request specifies the maximum duration for a gateway to respond to an HTTP request.
    /// If the gateway has not been able to respond before this deadline is met, the gateway
    /// MUST return a timeout error.
    ///
    /// For example, setting the `rules.timeouts.request` field to the value `10s` in an
    /// `HTTPRoute` will cause a timeout if a client request is taking longer than 10 seconds
    /// to complete.
    ///
    /// Setting a timeout to the zero duration (e.g. "0s") SHOULD disable the timeout
    /// completely. Implementations that cannot completely disable the timeout MUST
    /// instead interpret the zero duration as the longest possible value to which
    /// the timeout can be set.
    ///
    /// This timeout is intended to cover as close to the whole request-response transaction
    /// as possible although an implementation MAY choose to start the timeout after the entire
    /// request stream has been received instead of immediately after the transaction is
    /// initiated by the client.
    ///
    /// The value of Request is a Gateway API Duration string as defined by GEP-2257. When this
    /// field is unspecified, request timeout behavior is implementation-specific.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
}
