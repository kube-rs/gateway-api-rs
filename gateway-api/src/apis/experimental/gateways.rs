// WARNING: generated file - manual changes will be overriden

use super::common::*;
#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;
/// Spec defines the desired state of Gateway.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "Gateway",
    plural = "gateways"
)]
#[kube(namespaced)]
#[kube(status = "GatewayStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct GatewaySpec {
    /// Addresses requested for this Gateway. This is optional and behavior can
    /// depend on the implementation. If a value is set in the spec and the
    /// requested address is invalid or unavailable, the implementation MUST
    /// indicate this in the associated entry in GatewayStatus.Addresses.
    ///
    /// The Addresses field represents a request for the address(es) on the
    /// "outside of the Gateway", that traffic bound for this Gateway will use.
    /// This could be the IP address or hostname of an external load balancer or
    /// other networking infrastructure, or some other address that traffic will
    /// be sent to.
    ///
    /// If no Addresses are specified, the implementation MAY schedule the
    /// Gateway in an implementation-specific manner, assigning an appropriate
    /// set of Addresses.
    ///
    /// The implementation MUST bind all Listeners to every GatewayAddress that
    /// it assigns to the Gateway and add a corresponding entry in
    /// GatewayStatus.Addresses.
    ///
    /// Support: Extended
    ///
    ///
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewayAddress>>,
    /// BackendTLS configures TLS settings for when this Gateway is connecting to
    /// backends with TLS.
    ///
    /// Support: Core
    ///
    ///
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backendTLS"
    )]
    pub backend_tls: Option<GatewayBackendTls>,
    /// GatewayClassName used for this Gateway. This is the name of a
    /// GatewayClass resource.
    #[serde(rename = "gatewayClassName")]
    pub gateway_class_name: String,
    /// Infrastructure defines infrastructure level attributes about this Gateway instance.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<GatewayInfrastructure>,
    /// Listeners associated with this Gateway. Listeners define
    /// logical endpoints that are bound on this Gateway's addresses.
    /// At least one Listener MUST be specified.
    ///
    /// Each Listener in a set of Listeners (for example, in a single Gateway)
    /// MUST be _distinct_, in that a traffic flow MUST be able to be assigned to
    /// exactly one listener. (This section uses "set of Listeners" rather than
    /// "Listeners in a single Gateway" because implementations MAY merge configuration
    /// from multiple Gateways onto a single data plane, and these rules _also_
    /// apply in that case).
    ///
    /// Practically, this means that each listener in a set MUST have a unique
    /// combination of Port, Protocol, and, if supported by the protocol, Hostname.
    ///
    /// Some combinations of port, protocol, and TLS settings are considered
    /// Core support and MUST be supported by implementations based on their
    /// targeted conformance profile:
    ///
    /// HTTP Profile
    ///
    /// 1. HTTPRoute, Port: 80, Protocol: HTTP
    /// 2. HTTPRoute, Port: 443, Protocol: HTTPS, TLS Mode: Terminate, TLS keypair provided
    ///
    /// TLS Profile
    ///
    /// 1. TLSRoute, Port: 443, Protocol: TLS, TLS Mode: Passthrough
    ///
    /// "Distinct" Listeners have the following property:
    ///
    /// The implementation can match inbound requests to a single distinct
    /// Listener. When multiple Listeners share values for fields (for
    /// example, two Listeners with the same Port value), the implementation
    /// can match requests to only one of the Listeners using other
    /// Listener fields.
    ///
    /// For example, the following Listener scenarios are distinct:
    ///
    /// 1. Multiple Listeners with the same Port that all use the "HTTP"
    ///    Protocol that all have unique Hostname values.
    /// 2. Multiple Listeners with the same Port that use either the "HTTPS" or
    ///    "TLS" Protocol that all have unique Hostname values.
    /// 3. A mixture of "TCP" and "UDP" Protocol Listeners, where no Listener
    ///    with the same Protocol has the same Port value.
    ///
    /// Some fields in the Listener struct have possible values that affect
    /// whether the Listener is distinct. Hostname is particularly relevant
    /// for HTTP or HTTPS protocols.
    ///
    /// When using the Hostname value to select between same-Port, same-Protocol
    /// Listeners, the Hostname value must be different on each Listener for the
    /// Listener to be distinct.
    ///
    /// When the Listeners are distinct based on Hostname, inbound request
    /// hostnames MUST match from the most specific to least specific Hostname
    /// values to choose the correct Listener and its associated set of Routes.
    ///
    /// Exact matches must be processed before wildcard matches, and wildcard
    /// matches must be processed before fallback (empty Hostname value)
    /// matches. For example, `"foo.example.com"` takes precedence over
    /// `"*.example.com"`, and `"*.example.com"` takes precedence over `""`.
    ///
    /// Additionally, if there are multiple wildcard entries, more specific
    /// wildcard entries must be processed before less specific wildcard entries.
    /// For example, `"*.foo.example.com"` takes precedence over `"*.example.com"`.
    /// The precise definition here is that the higher the number of dots in the
    /// hostname to the right of the wildcard character, the higher the precedence.
    ///
    /// The wildcard character will match any number of characters _and dots_ to
    /// the left, however, so `"*.example.com"` will match both
    /// `"foo.bar.example.com"` _and_ `"bar.example.com"`.
    ///
    /// If a set of Listeners contains Listeners that are not distinct, then those
    /// Listeners are Conflicted, and the implementation MUST set the "Conflicted"
    /// condition in the Listener Status to "True".
    ///
    /// Implementations MAY choose to accept a Gateway with some Conflicted
    /// Listeners only if they only accept the partial Listener set that contains
    /// no Conflicted Listeners. To put this another way, implementations may
    /// accept a partial Listener set only if they throw out *all* the conflicting
    /// Listeners. No picking one of the conflicting listeners as the winner.
    /// This also means that the Gateway must have at least one non-conflicting
    /// Listener in this case, otherwise it violates the requirement that at
    /// least one Listener must be present.
    ///
    /// The implementation MUST set a "ListenersNotValid" condition on the
    /// Gateway Status when the Gateway contains Conflicted Listeners whether or
    /// not they accept the Gateway. That Condition SHOULD clearly
    /// indicate in the Message which Listeners are conflicted, and which are
    /// Accepted. Additionally, the Listener status for those listeners SHOULD
    /// indicate which Listeners are conflicted and not Accepted.
    ///
    /// A Gateway's Listeners are considered "compatible" if:
    ///
    /// 1. They are distinct.
    /// 2. The implementation can serve them in compliance with the Addresses
    ///    requirement that all Listeners are available on all assigned
    ///    addresses.
    ///
    /// Compatible combinations in Extended support are expected to vary across
    /// implementations. A combination that is compatible for one implementation
    /// may not be compatible for another.
    ///
    /// For example, an implementation that cannot serve both TCP and UDP listeners
    /// on the same address, or cannot mix HTTPS and generic TLS listens on the same port
    /// would not consider those cases compatible, even though they are distinct.
    ///
    /// Note that requests SHOULD match at most one Listener. For example, if
    /// Listeners are defined for "foo.example.com" and "*.example.com", a
    /// request to "foo.example.com" SHOULD only be routed using routes attached
    /// to the "foo.example.com" Listener (and not the "*.example.com" Listener).
    /// This concept is known as "Listener Isolation". Implementations that do
    /// not support Listener Isolation MUST clearly document this.
    ///
    /// Implementations MAY merge separate Gateways onto a single set of
    /// Addresses if all Listeners across all Gateways are compatible.
    ///
    /// Support: Core
    pub listeners: Vec<GatewayListeners>,
}
/// BackendTLS configures TLS settings for when this Gateway is connecting to
/// backends with TLS.
///
/// Support: Core
///
///
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayBackendTls {
    /// ClientCertificateRef is a reference to an object that contains a Client
    /// Certificate and the associated private key.
    ///
    /// References to a resource in different namespace are invalid UNLESS there
    /// is a ReferenceGrant in the target namespace that allows the certificate
    /// to be attached. If a ReferenceGrant does not allow this reference, the
    /// "ResolvedRefs" condition MUST be set to False for this listener with the
    /// "RefNotPermitted" reason.
    ///
    /// ClientCertificateRef can reference to standard Kubernetes resources, i.e.
    /// Secret, or implementation-specific custom resources.
    ///
    /// This setting can be overridden on the service level by use of BackendTLSPolicy.
    ///
    /// Support: Core
    ///
    ///
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "clientCertificateRef"
    )]
    pub client_certificate_ref: Option<BackendTlsClientCertificateReference>,
}
/// Infrastructure defines infrastructure level attributes about this Gateway instance.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayInfrastructure {
    /// Annotations that SHOULD be applied to any resources created in response to this Gateway.
    ///
    /// For implementations creating other Kubernetes objects, this should be the `metadata.annotations` field on resources.
    /// For other implementations, this refers to any relevant (implementation specific) "annotations" concepts.
    ///
    /// An implementation may chose to add additional implementation-specific annotations as they see fit.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels that SHOULD be applied to any resources created in response to this Gateway.
    ///
    /// For implementations creating other Kubernetes objects, this should be the `metadata.labels` field on resources.
    /// For other implementations, this refers to any relevant (implementation specific) "labels" concepts.
    ///
    /// An implementation may chose to add additional implementation-specific labels as they see fit.
    ///
    /// If an implementation maps these labels to Pods, or any other resource that would need to be recreated when labels
    /// change, it SHOULD clearly warn about this behavior in documentation.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// ParametersRef is a reference to a resource that contains the configuration
    /// parameters corresponding to the Gateway. This is optional if the
    /// controller does not require any additional configuration.
    ///
    /// This follows the same semantics as GatewayClass's `parametersRef`, but on a per-Gateway basis
    ///
    /// The Gateway's GatewayClass may provide its own `parametersRef`. When both are specified,
    /// the merging behavior is implementation specific.
    /// It is generally recommended that GatewayClass provides defaults that can be overridden by a Gateway.
    ///
    /// Support: Implementation-specific
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "parametersRef"
    )]
    pub parameters_ref: Option<GatewayInfrastructureParametersReference>,
}
/// Listener embodies the concept of a logical endpoint where a Gateway accepts
/// network connections.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayListeners {
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
    ///
    /// Support: Core
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedRoutes"
    )]
    pub allowed_routes: Option<GatewayListenersAllowedRoutes>,
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
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Name is the name of the Listener. This name MUST be unique within a
    /// Gateway.
    ///
    /// Support: Core
    pub name: String,
    /// Port is the network port. Multiple listeners may use the
    /// same port, subject to the Listener compatibility rules.
    ///
    /// Support: Core
    pub port: i32,
    /// Protocol specifies the network protocol this listener expects to receive.
    ///
    /// Support: Core
    pub protocol: String,
    /// TLS is the TLS configuration for the Listener. This field is required if
    /// the Protocol field is "HTTPS" or "TLS". It is invalid to set this field
    /// if the Protocol field is "HTTP", "TCP", or "UDP".
    ///
    /// The association of SNIs to Certificate defined in GatewayTLSConfig is
    /// defined based on the Hostname field for this listener.
    ///
    /// The GatewayClass MUST use the longest matching SNI out of all
    /// available certificates for any TLS handshake.
    ///
    /// Support: Core
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
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayListenersAllowedRoutes {
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
    pub namespaces: Option<GatewayListenersAllowedRoutesNamespaces>,
}
/// Namespaces indicates namespaces from which Routes may be attached to this
/// Listener. This is restricted to the namespace of this Gateway by default.
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayListenersAllowedRoutesNamespaces {
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
    pub from: Option<GatewayListenersAllowedRoutesNamespacesFrom>,
    /// Selector must be specified when From is set to "Selector". In that case,
    /// only Routes in Namespaces matching this Selector will be selected by this
    /// Gateway. This field is ignored for other values of "From".
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<GatewayListenersAllowedRoutesNamespacesSelector>,
}
/// Namespaces indicates namespaces from which Routes may be attached to this
/// Listener. This is restricted to the namespace of this Gateway by default.
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GatewayListenersAllowedRoutesNamespacesFrom {
    All,
    Selector,
    Same,
}
/// Selector must be specified when From is set to "Selector". In that case,
/// only Routes in Namespaces matching this Selector will be selected by this
/// Gateway. This field is ignored for other values of "From".
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayListenersAllowedRoutesNamespacesSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchExpressions"
    )]
    pub match_expressions:
        Option<Vec<GatewayListenersAllowedRoutesNamespacesSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "matchLabels"
    )]
    pub match_labels: Option<BTreeMap<String, String>>,
}
/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayListenersAllowedRoutesNamespacesSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}
/// TLS is the TLS configuration for the Listener. This field is required if
/// the Protocol field is "HTTPS" or "TLS". It is invalid to set this field
/// if the Protocol field is "HTTP", "TCP", or "UDP".
///
/// The association of SNIs to Certificate defined in GatewayTLSConfig is
/// defined based on the Hostname field for this listener.
///
/// The GatewayClass MUST use the longest matching SNI out of all
/// available certificates for any TLS handshake.
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayListenersTls {
    /// CertificateRefs contains a series of references to Kubernetes objects that
    /// contains TLS certificates and private keys. These certificates are used to
    /// establish a TLS handshake for requests that match the hostname of the
    /// associated listener.
    ///
    /// A single CertificateRef to a Kubernetes Secret has "Core" support.
    /// Implementations MAY choose to support attaching multiple certificates to
    /// a Listener, but this behavior is implementation-specific.
    ///
    /// References to a resource in different namespace are invalid UNLESS there
    /// is a ReferenceGrant in the target namespace that allows the certificate
    /// to be attached. If a ReferenceGrant does not allow this reference, the
    /// "ResolvedRefs" condition MUST be set to False for this listener with the
    /// "RefNotPermitted" reason.
    ///
    /// This field is required to have at least one element when the mode is set
    /// to "Terminate" (default) and is optional otherwise.
    ///
    /// CertificateRefs can reference to standard Kubernetes resources, i.e.
    /// Secret, or implementation-specific custom resources.
    ///
    /// Support: Core - A single reference to a Kubernetes Secret of type kubernetes.io/tls
    ///
    /// Support: Implementation-specific (More than one reference or other resource types)
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "certificateRefs"
    )]
    pub certificate_refs: Option<Vec<BackendTlsClientCertificateReference>>,
    /// FrontendValidation holds configuration information for validating the frontend (client).
    /// Setting this field will require clients to send a client certificate
    /// required for validation during the TLS handshake. In browsers this may result in a dialog appearing
    /// that requests a user to specify the client certificate.
    /// The maximum depth of a certificate chain accepted in verification is Implementation specific.
    ///
    /// Support: Extended
    ///
    ///
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "frontendValidation"
    )]
    pub frontend_validation: Option<GatewayListenersTlsFrontendValidation>,
    /// Mode defines the TLS behavior for the TLS session initiated by the client.
    /// There are two possible modes:
    ///
    /// - Terminate: The TLS session between the downstream client and the
    ///   Gateway is terminated at the Gateway. This mode requires certificates
    ///   to be specified in some way, such as populating the certificateRefs
    ///   field.
    /// - Passthrough: The TLS session is NOT terminated by the Gateway. This
    ///   implies that the Gateway can't decipher the TLS stream except for
    ///   the ClientHello message of the TLS protocol. The certificateRefs field
    ///   is ignored in this mode.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GatewayListenersTlsMode>,
    /// Options are a list of key/value pairs to enable extended TLS
    /// configuration for each implementation. For example, configuring the
    /// minimum TLS version or supported cipher suites.
    ///
    /// A set of common keys MAY be defined by the API in the future. To avoid
    /// any ambiguity, implementation-specific definitions MUST use
    /// domain-prefixed names, such as `example.com/my-custom-option`.
    /// Un-prefixed names are reserved for key names defined by Gateway API.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
}
/// FrontendValidation holds configuration information for validating the frontend (client).
/// Setting this field will require clients to send a client certificate
/// required for validation during the TLS handshake. In browsers this may result in a dialog appearing
/// that requests a user to specify the client certificate.
/// The maximum depth of a certificate chain accepted in verification is Implementation specific.
///
/// Support: Extended
///
///
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayListenersTlsFrontendValidation {
    /// CACertificateRefs contains one or more references to
    /// Kubernetes objects that contain TLS certificates of
    /// the Certificate Authorities that can be used
    /// as a trust anchor to validate the certificates presented by the client.
    ///
    /// A single CA certificate reference to a Kubernetes ConfigMap
    /// has "Core" support.
    /// Implementations MAY choose to support attaching multiple CA certificates to
    /// a Listener, but this behavior is implementation-specific.
    ///
    /// Support: Core - A single reference to a Kubernetes ConfigMap
    /// with the CA certificate in a key named `ca.crt`.
    ///
    /// Support: Implementation-specific (More than one reference, or other kinds
    /// of resources).
    ///
    /// References to a resource in a different namespace are invalid UNLESS there
    /// is a ReferenceGrant in the target namespace that allows the certificate
    /// to be attached. If a ReferenceGrant does not allow this reference, the
    /// "ResolvedRefs" condition MUST be set to False for this listener with the
    /// "RefNotPermitted" reason.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "caCertificateRefs"
    )]
    pub ca_certificate_refs: Option<Vec<ParametersReference>>,
}
/// TLS is the TLS configuration for the Listener. This field is required if
/// the Protocol field is "HTTPS" or "TLS". It is invalid to set this field
/// if the Protocol field is "HTTP", "TCP", or "UDP".
///
/// The association of SNIs to Certificate defined in GatewayTLSConfig is
/// defined based on the Hostname field for this listener.
///
/// The GatewayClass MUST use the longest matching SNI out of all
/// available certificates for any TLS handshake.
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GatewayListenersTlsMode {
    Terminate,
    Passthrough,
}
/// Status defines the current state of Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayStatus {
    /// Addresses lists the network addresses that have been bound to the
    /// Gateway.
    ///
    /// This list may differ from the addresses provided in the spec under some
    /// conditions:
    ///
    ///   * no addresses are specified, all addresses are dynamically assigned
    ///   * a combination of specified and dynamic addresses are assigned
    ///   * a specified address was unusable (e.g. already in use)
    ///
    ///
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewayAddress>>,
    /// Conditions describe the current conditions of the Gateway.
    ///
    /// Implementations should prefer to express Gateway conditions
    /// using the `GatewayConditionType` and `GatewayConditionReason`
    /// constants so that operators and tools can converge on a common
    /// vocabulary to describe Gateway state.
    ///
    /// Known condition types are:
    ///
    /// * "Accepted"
    /// * "Programmed"
    /// * "Ready"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Listeners provide status for each unique listener port defined in the Spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<GatewayStatusListeners>>,
}
/// ListenerStatus is the status associated with a Listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayStatusListeners {
    /// AttachedRoutes represents the total number of Routes that have been
    /// successfully attached to this Listener.
    ///
    /// Successful attachment of a Route to a Listener is based solely on the
    /// combination of the AllowedRoutes field on the corresponding Listener
    /// and the Route's ParentRefs field. A Route is successfully attached to
    /// a Listener when it is selected by the Listener's AllowedRoutes field
    /// AND the Route has a valid ParentRef selecting the whole Gateway
    /// resource or a specific Listener as a parent resource (more detail on
    /// attachment semantics can be found in the documentation on the various
    /// Route kinds ParentRefs fields). Listener or Route status does not impact
    /// successful attachment, i.e. the AttachedRoutes field count MUST be set
    /// for Listeners with condition Accepted: false and MUST count successfully
    /// attached Routes that may themselves have Accepted: false conditions.
    ///
    /// Uses for this field include troubleshooting Route attachment and
    /// measuring blast radius/impact of changes to a Listener.
    #[serde(rename = "attachedRoutes")]
    pub attached_routes: i32,
    /// Conditions describe the current condition of this listener.
    pub conditions: Vec<Condition>,
    /// Name is the name of the Listener that this status corresponds to.
    pub name: String,
    /// SupportedKinds is the list indicating the Kinds supported by this
    /// listener. This MUST represent the kinds an implementation supports for
    /// that Listener configuration.
    ///
    /// If kinds are specified in Spec that are not supported, they MUST NOT
    /// appear in this list and an implementation MUST set the "ResolvedRefs"
    /// condition to "False" with the "InvalidRouteKinds" reason. If both valid
    /// and invalid Route kinds are specified, the implementation MUST
    /// reference the valid Route kinds that have been specified.
    #[serde(rename = "supportedKinds")]
    pub supported_kinds: Vec<Kind>,
}
