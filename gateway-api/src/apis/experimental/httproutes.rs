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
/// Spec defines the desired state of HTTPRoute.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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
    /// Rules are a list of HTTP matchers, filters and actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<HTTPRouteRule>>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backendRefs"
    )]
    pub backend_refs: Option<Vec<HTTPBackendReference>>,
    /// Filters define the filters that are applied to requests that match
    /// this rule.
    ///
    /// Wherever possible, implementations SHOULD implement filters in the order
    /// they are specified.
    ///
    /// Implementations MAY choose to implement this ordering strictly, rejecting
    /// any combination or order of filters that cannot be supported. If implementations
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
    /// implementation cannot support other combinations of filters, they must clearly
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
    /// Name is the name of the route rule. This name MUST be unique within a Route if it is set.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Retry defines the configuration for when to retry an HTTP request.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<HTTPRouteRulesRetry>,
    /// SessionPersistence defines and configures session persistence
    /// for the route rule.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sessionPersistence"
    )]
    pub session_persistence: Option<SessionPersistence>,
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
    /// CORS defines a schema for a filter that responds to the
    /// cross-origin request based on HTTP response header.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<HTTPRouteRulesBackendRefsFiltersCors>,
    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// This filter can be used multiple times within the same rule.
    ///
    /// Support: Implementation-specific
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "extensionRef"
    )]
    pub extension_ref: Option<GatewayInfrastructureParametersReference>,
    /// ExternalAuth configures settings related to sending request details
    /// to an external auth service. The external service MUST authenticate
    /// the request, and MAY authorize the request as well.
    ///
    /// If there is any problem communicating with the external service,
    /// this filter MUST fail closed.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "externalAuth"
    )]
    pub external_auth: Option<HTTPRouteRulesBackendRefsFiltersExternalAuth>,
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
    pub request_mirror: Option<HTTPRouteRulesBackendRefsFiltersRequestMirror>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "urlRewrite"
    )]
    pub url_rewrite: Option<HTTPRouteUrlRewrite>,
}
/// CORS defines a schema for a filter that responds to the
/// cross-origin request based on HTTP response header.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRulesBackendRefsFiltersCors {
    /// AllowCredentials indicates whether the actual cross-origin request allows
    /// to include credentials.
    ///
    /// When set to true, the gateway will include the `Access-Control-Allow-Credentials`
    /// response header with value true (case-sensitive).
    ///
    /// When set to false or omitted the gateway will omit the header
    /// `Access-Control-Allow-Credentials` entirely (this is the standard CORS
    /// behavior).
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowCredentials"
    )]
    pub allow_credentials: Option<bool>,
    /// AllowHeaders indicates which HTTP request headers are supported for
    /// accessing the requested resource.
    ///
    /// Header names are not case sensitive.
    ///
    /// Multiple header names in the value of the `Access-Control-Allow-Headers`
    /// response header are separated by a comma (",").
    ///
    /// When the `AllowHeaders` field is configured with one or more headers, the
    /// gateway must return the `Access-Control-Allow-Headers` response header
    /// which value is present in the `AllowHeaders` field.
    ///
    /// If any header name in the `Access-Control-Request-Headers` request header
    /// is not included in the list of header names specified by the response
    /// header `Access-Control-Allow-Headers`, it will present an error on the
    /// client side.
    ///
    /// If any header name in the `Access-Control-Allow-Headers` response header
    /// does not recognize by the client, it will also occur an error on the
    /// client side.
    ///
    /// A wildcard indicates that the requests with all HTTP headers are allowed.
    /// The `Access-Control-Allow-Headers` response header can only use `*`
    /// wildcard as value when the `AllowCredentials` field is false or omitted.
    ///
    /// When the `AllowCredentials` field is true and `AllowHeaders` field
    /// specified with the `*` wildcard, the gateway must specify one or more
    /// HTTP headers in the value of the `Access-Control-Allow-Headers` response
    /// header. The value of the header `Access-Control-Allow-Headers` is same as
    /// the `Access-Control-Request-Headers` header provided by the client. If
    /// the header `Access-Control-Request-Headers` is not included in the
    /// request, the gateway will omit the `Access-Control-Allow-Headers`
    /// response header, instead of specifying the `*` wildcard. A Gateway
    /// implementation may choose to add implementation-specific default headers.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowHeaders"
    )]
    pub allow_headers: Option<Vec<String>>,
    /// AllowMethods indicates which HTTP methods are supported for accessing the
    /// requested resource.
    ///
    /// Valid values are any method defined by RFC9110, along with the special
    /// value `*`, which represents all HTTP methods are allowed.
    ///
    /// Method names are case sensitive, so these values are also case-sensitive.
    /// (See https://www.rfc-editor.org/rfc/rfc2616#section-5.1.1)
    ///
    /// Multiple method names in the value of the `Access-Control-Allow-Methods`
    /// response header are separated by a comma (",").
    ///
    /// A CORS-safelisted method is a method that is `GET`, `HEAD`, or `POST`.
    /// (See https://fetch.spec.whatwg.org/#cors-safelisted-method) The
    /// CORS-safelisted methods are always allowed, regardless of whether they
    /// are specified in the `AllowMethods` field.
    ///
    /// When the `AllowMethods` field is configured with one or more methods, the
    /// gateway must return the `Access-Control-Allow-Methods` response header
    /// which value is present in the `AllowMethods` field.
    ///
    /// If the HTTP method of the `Access-Control-Request-Method` request header
    /// is not included in the list of methods specified by the response header
    /// `Access-Control-Allow-Methods`, it will present an error on the client
    /// side.
    ///
    /// The `Access-Control-Allow-Methods` response header can only use `*`
    /// wildcard as value when the `AllowCredentials` field is false or omitted.
    ///
    /// When the `AllowCredentials` field is true and `AllowMethods` field
    /// specified with the `*` wildcard, the gateway must specify one HTTP method
    /// in the value of the Access-Control-Allow-Methods response header. The
    /// value of the header `Access-Control-Allow-Methods` is same as the
    /// `Access-Control-Request-Method` header provided by the client. If the
    /// header `Access-Control-Request-Method` is not included in the request,
    /// the gateway will omit the `Access-Control-Allow-Methods` response header,
    /// instead of specifying the `*` wildcard. A Gateway implementation may
    /// choose to add implementation-specific default methods.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowMethods"
    )]
    pub allow_methods: Option<Vec<String>>,
    /// AllowOrigins indicates whether the response can be shared with requested
    /// resource from the given `Origin`.
    ///
    /// The `Origin` consists of a scheme and a host, with an optional port, and
    /// takes the form `<scheme>://<host>(:<port>)`.
    ///
    /// Valid values for scheme are: `http` and `https`.
    ///
    /// Valid values for port are any integer between 1 and 65535 (the list of
    /// available TCP/UDP ports). Note that, if not included, port `80` is
    /// assumed for `http` scheme origins, and port `443` is assumed for `https`
    /// origins. This may affect origin matching.
    ///
    /// The host part of the origin may contain the wildcard character `*`. These
    /// wildcard characters behave as follows:
    ///
    /// * `*` is a greedy match to the _left_, including any number of
    ///   DNS labels to the left of its position. This also means that
    ///   `*` will include any number of period `.` characters to the
    ///   left of its position.
    /// * A wildcard by itself matches all hosts.
    ///
    /// An origin value that includes _only_ the `*` character indicates requests
    /// from all `Origin`s are allowed.
    ///
    /// When the `AllowOrigins` field is configured with multiple origins, it
    /// means the server supports clients from multiple origins. If the request
    /// `Origin` matches the configured allowed origins, the gateway must return
    /// the given `Origin` and sets value of the header
    /// `Access-Control-Allow-Origin` same as the `Origin` header provided by the
    /// client.
    ///
    /// The status code of a successful response to a "preflight" request is
    /// always an OK status (i.e., 204 or 200).
    ///
    /// If the request `Origin` does not match the configured allowed origins,
    /// the gateway returns 204/200 response but doesn't set the relevant
    /// cross-origin response headers. Alternatively, the gateway responds with
    /// 403 status to the "preflight" request is denied, coupled with omitting
    /// the CORS headers. The cross-origin request fails on the client side.
    /// Therefore, the client doesn't attempt the actual cross-origin request.
    ///
    /// The `Access-Control-Allow-Origin` response header can only use `*`
    /// wildcard as value when the `AllowCredentials` field is false or omitted.
    ///
    /// When the `AllowCredentials` field is true and `AllowOrigins` field
    /// specified with the `*` wildcard, the gateway must return a single origin
    /// in the value of the `Access-Control-Allow-Origin` response header,
    /// instead of specifying the `*` wildcard. The value of the header
    /// `Access-Control-Allow-Origin` is same as the `Origin` header provided by
    /// the client.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowOrigins"
    )]
    pub allow_origins: Option<Vec<String>>,
    /// ExposeHeaders indicates which HTTP response headers can be exposed
    /// to client-side scripts in response to a cross-origin request.
    ///
    /// A CORS-safelisted response header is an HTTP header in a CORS response
    /// that it is considered safe to expose to the client scripts.
    /// The CORS-safelisted response headers include the following headers:
    /// `Cache-Control`
    /// `Content-Language`
    /// `Content-Length`
    /// `Content-Type`
    /// `Expires`
    /// `Last-Modified`
    /// `Pragma`
    /// (See https://fetch.spec.whatwg.org/#cors-safelisted-response-header-name)
    /// The CORS-safelisted response headers are exposed to client by default.
    ///
    /// When an HTTP header name is specified using the `ExposeHeaders` field,
    /// this additional header will be exposed as part of the response to the
    /// client.
    ///
    /// Header names are not case sensitive.
    ///
    /// Multiple header names in the value of the `Access-Control-Expose-Headers`
    /// response header are separated by a comma (",").
    ///
    /// A wildcard indicates that the responses with all HTTP headers are exposed
    /// to clients. The `Access-Control-Expose-Headers` response header can only
    /// use `*` wildcard as value when the `AllowCredentials` field is false or omitted.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "exposeHeaders"
    )]
    pub expose_headers: Option<Vec<String>>,
    /// MaxAge indicates the duration (in seconds) for the client to cache the
    /// results of a "preflight" request.
    ///
    /// The information provided by the `Access-Control-Allow-Methods` and
    /// `Access-Control-Allow-Headers` response headers can be cached by the
    /// client until the time specified by `Access-Control-Max-Age` elapses.
    ///
    /// The default value of `Access-Control-Max-Age` response header is 5
    /// (seconds).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<i32>,
}
/// ExternalAuth configures settings related to sending request details
/// to an external auth service. The external service MUST authenticate
/// the request, and MAY authorize the request as well.
///
/// If there is any problem communicating with the external service,
/// this filter MUST fail closed.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRulesBackendRefsFiltersExternalAuth {
    /// BackendRef is a reference to a backend to send authorization
    /// requests to.
    ///
    /// The backend must speak the selected protocol (GRPC or HTTP) on the
    /// referenced port.
    ///
    /// If the backend service requires TLS, use BackendTLSPolicy to tell the
    /// implementation to supply the TLS details to be used to connect to that
    /// backend.
    #[serde(rename = "backendRef")]
    pub backend_ref: HTTPRouteRulesBackendRefsFiltersExternalAuthBackendRef,
    /// ForwardBody controls if requests to the authorization server should include
    /// the body of the client request; and if so, how big that body is allowed
    /// to be.
    ///
    /// It is expected that implementations will buffer the request body up to
    /// `forwardBody.maxSize` bytes. Bodies over that size must be rejected with a
    /// 4xx series error (413 or 403 are common examples), and fail processing
    /// of the filter.
    ///
    /// If unset, or `forwardBody.maxSize` is set to `0`, then the body will not
    /// be forwarded.
    ///
    /// Feature Name: HTTPRouteExternalAuthForwardBody
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "forwardBody"
    )]
    pub forward_body: Option<HTTPRouteRulesBackendRefsFiltersExternalAuthForwardBody>,
    /// GRPCAuthConfig contains configuration for communication with ext_authz
    /// protocol-speaking backends.
    ///
    /// If unset, implementations must assume the default behavior for each
    /// included field is intended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<HTTPRouteRulesBackendRefsFiltersExternalAuthGrpc>,
    /// HTTPAuthConfig contains configuration for communication with HTTP-speaking
    /// backends.
    ///
    /// If unset, implementations must assume the default behavior for each
    /// included field is intended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPRouteRulesBackendRefsFiltersExternalAuthHttp>,
    /// ExternalAuthProtocol describes which protocol to use when communicating with an
    /// ext_authz authorization server.
    ///
    /// When this is set to GRPC, each backend must use the Envoy ext_authz protocol
    /// on the port specified in `backendRefs`. Requests and responses are defined
    /// in the protobufs explained at:
    /// https://www.envoyproxy.io/docs/envoy/latest/api-v3/service/auth/v3/external_auth.proto
    ///
    /// When this is set to HTTP, each backend must respond with a `200` status
    /// code in on a successful authorization. Any other code is considered
    /// an authorization failure.
    ///
    /// Feature Names:
    /// GRPC Support - HTTPRouteExternalAuthGRPC
    /// HTTP Support - HTTPRouteExternalAuthHTTP
    pub protocol: HTTPRouteRulesBackendRefsFiltersExternalAuthProtocol,
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
pub struct HTTPRouteRulesBackendRefsFiltersRequestMirror {
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
    pub backend_ref: HTTPRouteRulesBackendRefsFiltersExternalAuthBackendRef,
    /// Fraction represents the fraction of requests that should be
    /// mirrored to BackendRef.
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fraction: Option<RequestMirrorFraction>,
    /// Percent represents the percentage of requests that should be
    /// mirrored to BackendRef. Its minimum value is 0 (indicating 0% of
    /// requests) and its maximum value is 100 (indicating 100% of requests).
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
}
/// HTTPRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. HTTPRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteFilter {
    /// CORS defines a schema for a filter that responds to the
    /// cross-origin request based on HTTP response header.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<HTTPRouteRulesFiltersCors>,
    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// This filter can be used multiple times within the same rule.
    ///
    /// Support: Implementation-specific
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "extensionRef"
    )]
    pub extension_ref: Option<GatewayInfrastructureParametersReference>,
    /// ExternalAuth configures settings related to sending request details
    /// to an external auth service. The external service MUST authenticate
    /// the request, and MAY authorize the request as well.
    ///
    /// If there is any problem communicating with the external service,
    /// this filter MUST fail closed.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "externalAuth"
    )]
    pub external_auth: Option<HTTPRouteRulesFiltersExternalAuth>,
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
    pub request_mirror: Option<HTTPRouteRulesFiltersRequestMirror>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "urlRewrite"
    )]
    pub url_rewrite: Option<HTTPRouteUrlRewrite>,
}
/// CORS defines a schema for a filter that responds to the
/// cross-origin request based on HTTP response header.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRulesFiltersCors {
    /// AllowCredentials indicates whether the actual cross-origin request allows
    /// to include credentials.
    ///
    /// When set to true, the gateway will include the `Access-Control-Allow-Credentials`
    /// response header with value true (case-sensitive).
    ///
    /// When set to false or omitted the gateway will omit the header
    /// `Access-Control-Allow-Credentials` entirely (this is the standard CORS
    /// behavior).
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowCredentials"
    )]
    pub allow_credentials: Option<bool>,
    /// AllowHeaders indicates which HTTP request headers are supported for
    /// accessing the requested resource.
    ///
    /// Header names are not case sensitive.
    ///
    /// Multiple header names in the value of the `Access-Control-Allow-Headers`
    /// response header are separated by a comma (",").
    ///
    /// When the `AllowHeaders` field is configured with one or more headers, the
    /// gateway must return the `Access-Control-Allow-Headers` response header
    /// which value is present in the `AllowHeaders` field.
    ///
    /// If any header name in the `Access-Control-Request-Headers` request header
    /// is not included in the list of header names specified by the response
    /// header `Access-Control-Allow-Headers`, it will present an error on the
    /// client side.
    ///
    /// If any header name in the `Access-Control-Allow-Headers` response header
    /// does not recognize by the client, it will also occur an error on the
    /// client side.
    ///
    /// A wildcard indicates that the requests with all HTTP headers are allowed.
    /// The `Access-Control-Allow-Headers` response header can only use `*`
    /// wildcard as value when the `AllowCredentials` field is false or omitted.
    ///
    /// When the `AllowCredentials` field is true and `AllowHeaders` field
    /// specified with the `*` wildcard, the gateway must specify one or more
    /// HTTP headers in the value of the `Access-Control-Allow-Headers` response
    /// header. The value of the header `Access-Control-Allow-Headers` is same as
    /// the `Access-Control-Request-Headers` header provided by the client. If
    /// the header `Access-Control-Request-Headers` is not included in the
    /// request, the gateway will omit the `Access-Control-Allow-Headers`
    /// response header, instead of specifying the `*` wildcard. A Gateway
    /// implementation may choose to add implementation-specific default headers.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowHeaders"
    )]
    pub allow_headers: Option<Vec<String>>,
    /// AllowMethods indicates which HTTP methods are supported for accessing the
    /// requested resource.
    ///
    /// Valid values are any method defined by RFC9110, along with the special
    /// value `*`, which represents all HTTP methods are allowed.
    ///
    /// Method names are case sensitive, so these values are also case-sensitive.
    /// (See https://www.rfc-editor.org/rfc/rfc2616#section-5.1.1)
    ///
    /// Multiple method names in the value of the `Access-Control-Allow-Methods`
    /// response header are separated by a comma (",").
    ///
    /// A CORS-safelisted method is a method that is `GET`, `HEAD`, or `POST`.
    /// (See https://fetch.spec.whatwg.org/#cors-safelisted-method) The
    /// CORS-safelisted methods are always allowed, regardless of whether they
    /// are specified in the `AllowMethods` field.
    ///
    /// When the `AllowMethods` field is configured with one or more methods, the
    /// gateway must return the `Access-Control-Allow-Methods` response header
    /// which value is present in the `AllowMethods` field.
    ///
    /// If the HTTP method of the `Access-Control-Request-Method` request header
    /// is not included in the list of methods specified by the response header
    /// `Access-Control-Allow-Methods`, it will present an error on the client
    /// side.
    ///
    /// The `Access-Control-Allow-Methods` response header can only use `*`
    /// wildcard as value when the `AllowCredentials` field is false or omitted.
    ///
    /// When the `AllowCredentials` field is true and `AllowMethods` field
    /// specified with the `*` wildcard, the gateway must specify one HTTP method
    /// in the value of the Access-Control-Allow-Methods response header. The
    /// value of the header `Access-Control-Allow-Methods` is same as the
    /// `Access-Control-Request-Method` header provided by the client. If the
    /// header `Access-Control-Request-Method` is not included in the request,
    /// the gateway will omit the `Access-Control-Allow-Methods` response header,
    /// instead of specifying the `*` wildcard. A Gateway implementation may
    /// choose to add implementation-specific default methods.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowMethods"
    )]
    pub allow_methods: Option<Vec<String>>,
    /// AllowOrigins indicates whether the response can be shared with requested
    /// resource from the given `Origin`.
    ///
    /// The `Origin` consists of a scheme and a host, with an optional port, and
    /// takes the form `<scheme>://<host>(:<port>)`.
    ///
    /// Valid values for scheme are: `http` and `https`.
    ///
    /// Valid values for port are any integer between 1 and 65535 (the list of
    /// available TCP/UDP ports). Note that, if not included, port `80` is
    /// assumed for `http` scheme origins, and port `443` is assumed for `https`
    /// origins. This may affect origin matching.
    ///
    /// The host part of the origin may contain the wildcard character `*`. These
    /// wildcard characters behave as follows:
    ///
    /// * `*` is a greedy match to the _left_, including any number of
    ///   DNS labels to the left of its position. This also means that
    ///   `*` will include any number of period `.` characters to the
    ///   left of its position.
    /// * A wildcard by itself matches all hosts.
    ///
    /// An origin value that includes _only_ the `*` character indicates requests
    /// from all `Origin`s are allowed.
    ///
    /// When the `AllowOrigins` field is configured with multiple origins, it
    /// means the server supports clients from multiple origins. If the request
    /// `Origin` matches the configured allowed origins, the gateway must return
    /// the given `Origin` and sets value of the header
    /// `Access-Control-Allow-Origin` same as the `Origin` header provided by the
    /// client.
    ///
    /// The status code of a successful response to a "preflight" request is
    /// always an OK status (i.e., 204 or 200).
    ///
    /// If the request `Origin` does not match the configured allowed origins,
    /// the gateway returns 204/200 response but doesn't set the relevant
    /// cross-origin response headers. Alternatively, the gateway responds with
    /// 403 status to the "preflight" request is denied, coupled with omitting
    /// the CORS headers. The cross-origin request fails on the client side.
    /// Therefore, the client doesn't attempt the actual cross-origin request.
    ///
    /// The `Access-Control-Allow-Origin` response header can only use `*`
    /// wildcard as value when the `AllowCredentials` field is false or omitted.
    ///
    /// When the `AllowCredentials` field is true and `AllowOrigins` field
    /// specified with the `*` wildcard, the gateway must return a single origin
    /// in the value of the `Access-Control-Allow-Origin` response header,
    /// instead of specifying the `*` wildcard. The value of the header
    /// `Access-Control-Allow-Origin` is same as the `Origin` header provided by
    /// the client.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowOrigins"
    )]
    pub allow_origins: Option<Vec<String>>,
    /// ExposeHeaders indicates which HTTP response headers can be exposed
    /// to client-side scripts in response to a cross-origin request.
    ///
    /// A CORS-safelisted response header is an HTTP header in a CORS response
    /// that it is considered safe to expose to the client scripts.
    /// The CORS-safelisted response headers include the following headers:
    /// `Cache-Control`
    /// `Content-Language`
    /// `Content-Length`
    /// `Content-Type`
    /// `Expires`
    /// `Last-Modified`
    /// `Pragma`
    /// (See https://fetch.spec.whatwg.org/#cors-safelisted-response-header-name)
    /// The CORS-safelisted response headers are exposed to client by default.
    ///
    /// When an HTTP header name is specified using the `ExposeHeaders` field,
    /// this additional header will be exposed as part of the response to the
    /// client.
    ///
    /// Header names are not case sensitive.
    ///
    /// Multiple header names in the value of the `Access-Control-Expose-Headers`
    /// response header are separated by a comma (",").
    ///
    /// A wildcard indicates that the responses with all HTTP headers are exposed
    /// to clients. The `Access-Control-Expose-Headers` response header can only
    /// use `*` wildcard as value when the `AllowCredentials` field is false or omitted.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "exposeHeaders"
    )]
    pub expose_headers: Option<Vec<String>>,
    /// MaxAge indicates the duration (in seconds) for the client to cache the
    /// results of a "preflight" request.
    ///
    /// The information provided by the `Access-Control-Allow-Methods` and
    /// `Access-Control-Allow-Headers` response headers can be cached by the
    /// client until the time specified by `Access-Control-Max-Age` elapses.
    ///
    /// The default value of `Access-Control-Max-Age` response header is 5
    /// (seconds).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<i32>,
}
/// ExternalAuth configures settings related to sending request details
/// to an external auth service. The external service MUST authenticate
/// the request, and MAY authorize the request as well.
///
/// If there is any problem communicating with the external service,
/// this filter MUST fail closed.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRulesFiltersExternalAuth {
    /// BackendRef is a reference to a backend to send authorization
    /// requests to.
    ///
    /// The backend must speak the selected protocol (GRPC or HTTP) on the
    /// referenced port.
    ///
    /// If the backend service requires TLS, use BackendTLSPolicy to tell the
    /// implementation to supply the TLS details to be used to connect to that
    /// backend.
    #[serde(rename = "backendRef")]
    pub backend_ref: HTTPRouteRulesBackendRefsFiltersExternalAuthBackendRef,
    /// ForwardBody controls if requests to the authorization server should include
    /// the body of the client request; and if so, how big that body is allowed
    /// to be.
    ///
    /// It is expected that implementations will buffer the request body up to
    /// `forwardBody.maxSize` bytes. Bodies over that size must be rejected with a
    /// 4xx series error (413 or 403 are common examples), and fail processing
    /// of the filter.
    ///
    /// If unset, or `forwardBody.maxSize` is set to `0`, then the body will not
    /// be forwarded.
    ///
    /// Feature Name: HTTPRouteExternalAuthForwardBody
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "forwardBody"
    )]
    pub forward_body: Option<HTTPRouteRulesBackendRefsFiltersExternalAuthForwardBody>,
    /// GRPCAuthConfig contains configuration for communication with ext_authz
    /// protocol-speaking backends.
    ///
    /// If unset, implementations must assume the default behavior for each
    /// included field is intended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<HTTPRouteRulesBackendRefsFiltersExternalAuthGrpc>,
    /// HTTPAuthConfig contains configuration for communication with HTTP-speaking
    /// backends.
    ///
    /// If unset, implementations must assume the default behavior for each
    /// included field is intended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<HTTPRouteRulesBackendRefsFiltersExternalAuthHttp>,
    /// ExternalAuthProtocol describes which protocol to use when communicating with an
    /// ext_authz authorization server.
    ///
    /// When this is set to GRPC, each backend must use the Envoy ext_authz protocol
    /// on the port specified in `backendRefs`. Requests and responses are defined
    /// in the protobufs explained at:
    /// https://www.envoyproxy.io/docs/envoy/latest/api-v3/service/auth/v3/external_auth.proto
    ///
    /// When this is set to HTTP, each backend must respond with a `200` status
    /// code in on a successful authorization. Any other code is considered
    /// an authorization failure.
    ///
    /// Feature Names:
    /// GRPC Support - HTTPRouteExternalAuthGRPC
    /// HTTP Support - HTTPRouteExternalAuthHTTP
    pub protocol: HTTPRouteRulesBackendRefsFiltersExternalAuthProtocol,
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
pub struct HTTPRouteRulesFiltersRequestMirror {
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
    pub backend_ref: HTTPRouteRulesBackendRefsFiltersExternalAuthBackendRef,
    /// Fraction represents the fraction of requests that should be
    /// mirrored to BackendRef.
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fraction: Option<RequestMirrorFraction>,
    /// Percent represents the percentage of requests that should be
    /// mirrored to BackendRef. Its minimum value is 0 (indicating 0% of
    /// requests) and its maximum value is 100 (indicating 100% of requests).
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "queryParams"
    )]
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
/// Retry defines the configuration for when to retry an HTTP request.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRulesRetry {
    /// Attempts specifies the maximum number of times an individual request
    /// from the gateway to a backend should be retried.
    ///
    /// If the maximum number of retries has been attempted without a successful
    /// response from the backend, the Gateway MUST return an error.
    ///
    /// When this field is unspecified, the number of times to attempt to retry
    /// a backend request is implementation-specific.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
    /// Backoff specifies the minimum duration a Gateway should wait between
    /// retry attempts and is represented in Gateway API Duration formatting.
    ///
    /// For example, setting the `rules[].retry.backoff` field to the value
    /// `100ms` will cause a backend request to first be retried approximately
    /// 100 milliseconds after timing out or receiving a response code configured
    /// to be retryable.
    ///
    /// An implementation MAY use an exponential or alternative backoff strategy
    /// for subsequent retry attempts, MAY cap the maximum backoff duration to
    /// some amount greater than the specified minimum, and MAY add arbitrary
    /// jitter to stagger requests, as long as unsuccessful backend requests are
    /// not retried before the configured minimum duration.
    ///
    /// If a Request timeout (`rules[].timeouts.request`) is configured on the
    /// route, the entire duration of the initial request and any retry attempts
    /// MUST not exceed the Request timeout duration. If any retry attempts are
    /// still in progress when the Request timeout duration has been reached,
    /// these SHOULD be canceled if possible and the Gateway MUST immediately
    /// return a timeout error.
    ///
    /// If a BackendRequest timeout (`rules[].timeouts.backendRequest`) is
    /// configured on the route, any retry attempts which reach the configured
    /// BackendRequest timeout duration without a response SHOULD be canceled if
    /// possible and the Gateway should wait for at least the specified backoff
    /// duration before attempting to retry the backend request again.
    ///
    /// If a BackendRequest timeout is _not_ configured on the route, retry
    /// attempts MAY time out after an implementation default duration, or MAY
    /// remain pending until a configured Request timeout or implementation
    /// default duration for total request time is reached.
    ///
    /// When this field is unspecified, the time to wait between retry attempts
    /// is implementation-specific.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff: Option<String>,
    /// Codes defines the HTTP response status codes for which a backend request
    /// should be retried.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codes: Option<Vec<i64>>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backendRequest"
    )]
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
