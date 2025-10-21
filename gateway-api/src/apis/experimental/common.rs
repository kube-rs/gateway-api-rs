// WARNING: generated file - manual changes will be overriden

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube_derive::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GRPCFilterType {
    ResponseHeaderModifier,
    RequestHeaderModifier,
    RequestMirror,
    ExtensionRef,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GatewayDefaultScope {
    All,
    None,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GatewayTlsFrontendDefaultValidationMode {
    AllowValidOnly,
    AllowInsecureFallback,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HTTPFilterType {
    RequestHeaderModifier,
    ResponseHeaderModifier,
    RequestMirror,
    RequestRedirect,
    #[serde(rename = "URLRewrite")]
    UrlRewrite,
    ExtensionRef,
    #[serde(rename = "CORS")]
    Cors,
    ExternalAuth,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HTTPRouteRulesBackendRefsFiltersExternalAuthProtocol {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "GRPC")]
    Grpc,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HeaderMatchType {
    Exact,
    RegularExpression,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum PersistenceCookieConfigLifetime {
    Permanent,
    Session,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum RedirectStatusCode {
    #[serde(rename = "301")]
    r#_301,
    #[serde(rename = "302")]
    r#_302,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum RequestOperationType {
    ReplaceFullPath,
    ReplacePrefixMatch,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum RequestRedirectScheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum SessionPersistenceType {
    Cookie,
    Header,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendTlsClientCertificateReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayAllowedListenersNamespacesSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayInfrastructureParametersReference {
    pub group: String,
    pub kind: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPHeader {
    pub name: String,
    pub value: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRulesBackendRefsFiltersExternalAuthBackendRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRulesBackendRefsFiltersExternalAuthForwardBody {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSize")]
    pub max_size: Option<i64>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRulesBackendRefsFiltersExternalAuthGrpc {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedHeaders"
    )]
    pub allowed_headers: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteRulesBackendRefsFiltersExternalAuthHttp {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedHeaders"
    )]
    pub allowed_headers: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedResponseHeaders"
    )]
    pub allowed_response_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct Kind {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub kind: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ParametersReference {
    pub group: String,
    pub kind: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ParentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sectionName"
    )]
    pub section_name: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RequestMirrorFraction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denominator: Option<i32>,
    pub numerator: i32,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct CommonRouteRule {
    #[serde(rename = "backendRefs")]
    pub backend_refs: Vec<BackendReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HeaderMatch {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<HeaderMatchType>,
    pub value: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HeaderModifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<HTTPHeader>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<HTTPHeader>>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ParentRouteStatus {
    pub conditions: Vec<Condition>,
    #[serde(rename = "controllerName")]
    pub controller_name: String,
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentReference,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RequestRedirectPath {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replaceFullPath"
    )]
    pub replace_full_path: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "replacePrefixMatch"
    )]
    pub replace_prefix_match: Option<String>,
    #[serde(rename = "type")]
    pub r#type: RequestOperationType,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct SessionPersistenceCookieConfig {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lifetimeType"
    )]
    pub lifetime_type: Option<PersistenceCookieConfigLifetime>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RequestRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<RequestRedirectPath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<RequestRedirectScheme>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "statusCode"
    )]
    pub status_code: Option<i64>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteUrlRewrite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<RequestRedirectPath>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RouteStatus {
    pub parents: Vec<ParentRouteStatus>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct SessionPersistence {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "absoluteTimeout"
    )]
    pub absolute_timeout: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cookieConfig"
    )]
    pub cookie_config: Option<SessionPersistenceCookieConfig>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "idleTimeout"
    )]
    pub idle_timeout: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sessionName"
    )]
    pub session_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<SessionPersistenceType>,
}
