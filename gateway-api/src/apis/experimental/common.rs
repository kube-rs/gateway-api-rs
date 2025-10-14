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
pub enum GrpcRouteRulesBackendRefsFiltersType {
    ResponseHeaderModifier,
    RequestHeaderModifier,
    RequestMirror,
    ExtensionRef,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpRouteRulesBackendRefsFiltersExternalAuthProtocol {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "GRPC")]
    Grpc,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpRouteRulesBackendRefsFiltersRequestRedirectPathType {
    ReplaceFullPath,
    ReplacePrefixMatch,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpRouteRulesBackendRefsFiltersRequestRedirectScheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpRouteRulesBackendRefsFiltersRequestRedirectStatusCode {
    #[serde(rename = "301")]
    r#_301,
    #[serde(rename = "302")]
    r#_302,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpRouteRulesBackendRefsFiltersType {
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
pub enum HttpRouteRulesMatchesHeadersType {
    Exact,
    RegularExpression,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpRouteRulesSessionPersistenceCookieConfigLifetimeType {
    Permanent,
    Session,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpRouteRulesSessionPersistenceType {
    Cookie,
    Header,
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
pub struct HttpRouteParentRefs {
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
pub struct HttpRouteRulesBackendRefsFiltersExternalAuthBackendRef {
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
pub struct HttpRouteRulesBackendRefsFiltersExternalAuthForwardBody {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSize")]
    pub max_size: Option<i64>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRouteRulesBackendRefsFiltersExternalAuthGrpc {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "allowedHeaders"
    )]
    pub allowed_headers: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRouteRulesBackendRefsFiltersExternalAuthHttp {
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
pub struct HttpRouteRulesBackendRefsFiltersRequestHeaderModifierAdd {
    pub name: String,
    pub value: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRouteRulesBackendRefsFiltersRequestMirrorFraction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denominator: Option<i32>,
    pub numerator: i32,
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
pub struct TcpRouteRulesBackendRefs {
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
