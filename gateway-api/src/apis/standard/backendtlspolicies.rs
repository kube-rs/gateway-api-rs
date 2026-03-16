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
/// Spec defines the desired state of BackendTLSPolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "BackendTLSPolicy",
    plural = "backendtlspolicies"
)]
#[kube(namespaced)]
#[kube(status = "BackendTlsPolicyStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct BackendTlsPolicySpec {
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
    /// TargetRefs identifies an API object to apply the policy to.
    /// Note that this config applies to the entire referenced resource
    /// by default, but this default may change in the future to provide
    /// a more granular application of the policy.
    ///
    /// TargetRefs must be _distinct_. This means either that:
    ///
    /// * They select different targets. If this is the case, then targetRef
    ///   entries are distinct. In terms of fields, this means that the
    ///   multi-part key defined by `group`, `kind`, and `name` must
    ///   be unique across all targetRef entries in the BackendTLSPolicy.
    /// * They select different sectionNames in the same target.
    ///
    /// When more than one BackendTLSPolicy selects the same target and
    /// sectionName, implementations MUST determine precedence using the
    /// following criteria, continuing on ties:
    ///
    /// * The older policy by creation timestamp takes precedence. For
    ///   example, a policy with a creation timestamp of "2021-07-15
    ///   01:02:03" MUST be given precedence over a policy with a
    ///   creation timestamp of "2021-07-15 01:02:04".
    /// * The policy appearing first in alphabetical order by {namespace}/{name}.
    ///   For example, a policy named `foo/bar` is given precedence over a
    ///   policy named `foo/baz`.
    ///
    /// For any BackendTLSPolicy that does not take precedence, the
    /// implementation MUST ensure the `Accepted` Condition is set to
    /// `status: False`, with Reason `Conflicted`.
    ///
    /// Implementations SHOULD NOT support more than one targetRef at this
    /// time. Although the API technically allows for this, the current guidance
    /// for conflict resolution and status handling is lacking. Until that can be
    /// clarified in a future release, the safest approach is to support a single
    /// targetRef.
    ///
    /// Support Levels:
    ///
    /// * Extended: Kubernetes Service referenced by HTTPRoute backendRefs.
    ///
    /// * Implementation-Specific: Services not connected via HTTPRoute, and any
    ///   other kind of backend. Implementations MAY use BackendTLSPolicy for:
    ///   - Services not referenced by any Route (e.g., infrastructure services)
    ///   - Gateway feature backends (e.g., ExternalAuth, rate-limiting services)
    ///   - Service mesh workload-to-service communication
    ///   - Other resource types beyond Service
    ///
    /// Implementations SHOULD aim to ensure that BackendTLSPolicy behavior is consistent,
    /// even outside of the extended HTTPRoute -(backendRef) -> Service path.
    /// They SHOULD clearly document how BackendTLSPolicy is interpreted in these
    /// scenarios, including:
    ///   - Which resources beyond Service are supported
    ///   - How the policy is discovered and applied
    ///   - Any implementation-specific semantics or restrictions
    ///
    /// Note that this config applies to the entire referenced resource
    /// by default, but this default may change in the future to provide
    /// a more granular application of the policy.
    #[serde(rename = "targetRefs")]
    pub target_refs: Vec<BackendTlsPolicyTargetRefs>,
    /// Validation contains backend TLS validation configuration.
    pub validation: BackendTlsPolicyValidation,
}
/// LocalPolicyTargetReferenceWithSectionName identifies an API object to apply a
/// direct policy to. This should be used as part of Policy resources that can
/// target single resources. For more information on how this policy attachment
/// mode works, and a sample Policy resource, refer to the policy attachment
/// documentation for Gateway API.
///
/// Note: This should only be used for direct policy attachment when references
/// to SectionName are actually needed. In all other cases,
/// LocalPolicyTargetReference should be used.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendTlsPolicyTargetRefs {
    /// Group is the group of the target resource.
    pub group: String,
    /// Kind is kind of the target resource.
    pub kind: String,
    /// Name is the name of the target resource.
    pub name: String,
    /// SectionName is the name of a section within the target resource. When
    /// unspecified, this targetRef targets the entire resource. In the following
    /// resources, SectionName is interpreted as the following:
    ///
    /// * Gateway: Listener name
    /// * HTTPRoute: HTTPRouteRule name
    /// * Service: Port name
    ///
    /// If a SectionName is specified, but does not exist on the targeted object,
    /// the Policy must fail to attach, and the policy implementation should record
    /// a `ResolvedRefs` or similar Condition in the Policy's status.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sectionName"
    )]
    pub section_name: Option<String>,
}
/// Validation contains backend TLS validation configuration.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendTlsPolicyValidation {
    /// CACertificateRefs contains one or more references to Kubernetes objects that
    /// contain a PEM-encoded TLS CA certificate bundle, which is used to
    /// validate a TLS handshake between the Gateway and backend Pod.
    ///
    /// If CACertificateRefs is empty or unspecified, then WellKnownCACertificates must be
    /// specified. Only one of CACertificateRefs or WellKnownCACertificates may be specified,
    /// not both. If CACertificateRefs is empty or unspecified, the configuration for
    /// WellKnownCACertificates MUST be honored instead if supported by the implementation.
    ///
    /// A CACertificateRef is invalid if:
    ///
    /// * It refers to a resource that cannot be resolved (e.g., the referenced resource
    ///   does not exist) or is misconfigured (e.g., a ConfigMap does not contain a key
    ///   named `ca.crt`). In this case, the Reason must be set to `InvalidCACertificateRef`
    ///   and the Message of the Condition must indicate which reference is invalid and why.
    ///
    /// * It refers to an unknown or unsupported kind of resource. In this case, the Reason
    ///   must be set to `InvalidKind` and the Message of the Condition must explain which
    ///   kind of resource is unknown or unsupported.
    ///
    /// * It refers to a resource in another namespace. This may change in future
    ///   spec updates.
    ///
    /// Implementations MAY choose to perform further validation of the certificate
    /// content (e.g., checking expiry or enforcing specific formats). In such cases,
    /// an implementation-specific Reason and Message must be set for the invalid reference.
    ///
    /// In all cases, the implementation MUST ensure the `ResolvedRefs` Condition on
    /// the BackendTLSPolicy is set to `status: False`, with a Reason and Message
    /// that indicate the cause of the error. Connections using an invalid
    /// CACertificateRef MUST fail, and the client MUST receive an HTTP 5xx error
    /// response. If ALL CACertificateRefs are invalid, the implementation MUST also
    /// ensure the `Accepted` Condition on the BackendTLSPolicy is set to
    /// `status: False`, with a Reason `NoValidCACertificate`.
    ///
    /// A single CACertificateRef to a Kubernetes ConfigMap kind has "Core" support.
    /// Implementations MAY choose to support attaching multiple certificates to
    /// a backend, but this behavior is implementation-specific.
    ///
    /// Support: Core - An optional single reference to a Kubernetes ConfigMap,
    /// with the CA certificate in a key named `ca.crt`.
    ///
    /// Support: Implementation-specific - More than one reference, other kinds
    /// of resources, or a single reference that includes multiple certificates.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "caCertificateRefs"
    )]
    pub ca_certificate_refs: Option<Vec<ExtensionParametersReference>>,
    /// Hostname is used for two purposes in the connection between Gateways and
    /// backends:
    ///
    /// 1. Hostname MUST be used as the SNI to connect to the backend (RFC 6066).
    /// 2. Hostname MUST be used for authentication and MUST match the certificate
    ///    served by the matching backend, unless SubjectAltNames is specified.
    /// 3. If SubjectAltNames are specified, Hostname can be used for certificate selection
    ///    but MUST NOT be used for authentication. If you want to use the value
    ///    of the Hostname field for authentication, you MUST add it to the SubjectAltNames list.
    ///
    /// Support: Core
    pub hostname: String,
    /// SubjectAltNames contains one or more Subject Alternative Names.
    /// When specified the certificate served from the backend MUST
    /// have at least one Subject Alternate Name matching one of the specified SubjectAltNames.
    ///
    /// Support: Extended
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subjectAltNames"
    )]
    pub subject_alt_names: Option<Vec<BackendTlsPolicyValidationSubjectAltNames>>,
    /// WellKnownCACertificates specifies whether a well-known set of CA certificates
    /// may be used in the TLS handshake between the gateway and backend pod.
    ///
    /// If WellKnownCACertificates is unspecified or empty (""), then CACertificateRefs
    /// must be specified with at least one entry for a valid configuration. Only one of
    /// CACertificateRefs or WellKnownCACertificates may be specified, not both.
    /// If an implementation does not support the WellKnownCACertificates field, or
    /// the supplied value is not recognized, the implementation MUST ensure the
    /// `Accepted` Condition on the BackendTLSPolicy is set to `status: False`, with
    /// a Reason `Invalid`.
    ///
    /// Valid values include:
    /// * "System" - indicates that well-known system CA certificates should be used.
    ///
    /// Implementations MAY define their own sets of CA certificates. Such definitions
    /// MUST use an implementation-specific, prefixed name, such as
    /// `mycompany.com/my-custom-ca-certificates`.
    ///
    /// Support: Implementation-specific
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wellKnownCACertificates"
    )]
    pub well_known_ca_certificates: Option<String>,
}
/// SubjectAltName represents Subject Alternative Name.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendTlsPolicyValidationSubjectAltNames {
    /// Hostname contains Subject Alternative Name specified in DNS name format.
    /// Required when Type is set to Hostname, ignored otherwise.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Type determines the format of the Subject Alternative Name. Always required.
    ///
    /// Support: Core
    #[serde(rename = "type")]
    pub r#type: BackendTlsPolicyValidationSubjectAltNamesType,
    /// URI contains Subject Alternative Name specified in a full URI format.
    /// It MUST include both a scheme (e.g., "http" or "ftp") and a scheme-specific-part.
    /// Common values include SPIFFE IDs like "spiffe://mycluster.example.com/ns/myns/sa/svc1sa".
    /// Required when Type is set to URI, ignored otherwise.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
/// SubjectAltName represents Subject Alternative Name.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum BackendTlsPolicyValidationSubjectAltNamesType {
    Hostname,
    #[serde(rename = "URI")]
    Uri,
}
/// Status defines the current state of BackendTLSPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendTlsPolicyStatus {
    /// Ancestors is a list of ancestor resources (usually Gateways) that are
    /// associated with the policy, and the status of the policy with respect to
    /// each ancestor. When this policy attaches to a parent, the controller that
    /// manages the parent and the ancestors MUST add an entry to this list when
    /// the controller first sees the policy and SHOULD update the entry as
    /// appropriate when the relevant ancestor is modified.
    ///
    /// Note that choosing the relevant ancestor is left to the Policy designers;
    /// an important part of Policy design is designing the right object level at
    /// which to namespace this status.
    ///
    /// Note also that implementations MUST ONLY populate ancestor status for
    /// the Ancestor resources they are responsible for. Implementations MUST
    /// use the ControllerName field to uniquely identify the entries in this list
    /// that they are responsible for.
    ///
    /// Note that to achieve this, the list of PolicyAncestorStatus structs
    /// MUST be treated as a map with a composite key, made up of the AncestorRef
    /// and ControllerName fields combined.
    ///
    /// A maximum of 16 ancestors will be represented in this list. An empty list
    /// means the Policy is not relevant for any ancestors.
    ///
    /// If this slice is full, implementations MUST NOT add further entries.
    /// Instead they MUST consider the policy unimplementable and signal that
    /// on any related resources such as the ancestor that would be referenced
    /// here. For example, if this list was full on BackendTLSPolicy, no
    /// additional Gateways would be able to reference the Service targeted by
    /// the BackendTLSPolicy.
    pub ancestors: Vec<BackendTlsPolicyStatusAncestors>,
}
/// PolicyAncestorStatus describes the status of a route with respect to an
/// associated Ancestor.
///
/// Ancestors refer to objects that are either the Target of a policy or above it
/// in terms of object hierarchy. For example, if a policy targets a Service, the
/// Policy's Ancestors are, in order, the Service, the HTTPRoute, the Gateway, and
/// the GatewayClass. Almost always, in this hierarchy, the Gateway will be the most
/// useful object to place Policy status on, so we recommend that implementations
/// SHOULD use Gateway as the PolicyAncestorStatus object unless the designers
/// have a _very_ good reason otherwise.
///
/// In the context of policy attachment, the Ancestor is used to distinguish which
/// resource results in a distinct application of this policy. For example, if a policy
/// targets a Service, it may have a distinct result per attached Gateway.
///
/// Policies targeting the same resource may have different effects depending on the
/// ancestors of those resources. For example, different Gateways targeting the same
/// Service may have different capabilities, especially if they have different underlying
/// implementations.
///
/// For example, in BackendTLSPolicy, the Policy attaches to a Service that is
/// used as a backend in a HTTPRoute that is itself attached to a Gateway.
/// In this case, the relevant object for status is the Gateway, and that is the
/// ancestor object referred to in this status.
///
/// Note that a parent is also an ancestor, so for objects where the parent is the
/// relevant object for status, this struct SHOULD still be used.
///
/// This struct is intended to be used in a slice that's effectively a map,
/// with a composite key made up of the AncestorRef and the ControllerName.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendTlsPolicyStatusAncestors {
    /// AncestorRef corresponds with a ParentRef in the spec that this
    /// PolicyAncestorStatus struct describes the status of.
    #[serde(rename = "ancestorRef")]
    pub ancestor_ref: ParentReference,
    /// Conditions describes the status of the Policy with respect to the given Ancestor.
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
}
