// WARNING! generated file do not edit

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayAddress {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    pub value: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayInfrastructureParametersRef {
    pub group: String,
    pub kind: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct Kind {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub kind: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPHeader {
    pub name: String,
    pub value: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RequestMirrorRef {
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
pub struct RouteRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
}
/// HTTPRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. HTTPRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HTTPFilterType {
    RequestHeaderModifier,
    ResponseHeaderModifier,
    RequestMirror,
    RequestRedirect,
    #[serde(rename = "URLRewrite")]
    UrlRewrite,
    ExtensionRef,
}
/// GRPCRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. GRPCRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GRPCFilterType {
    ResponseHeaderModifier,
    RequestHeaderModifier,
    RequestMirror,
    ExtensionRef,
}
/// RequestRedirect defines a schema for a filter that responds to the
/// request with an HTTP redirection.
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum RequestRedirectScheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}
/// GRPCHeaderMatch describes how to select a gRPC route by matching gRPC request
/// headers.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HeaderMatchType {
    Exact,
    RegularExpression,
}
/// Path defines parameters used to modify the path of the incoming request.
/// The modified path is then used to construct the `Location` header. When
/// empty, the request path is used as-is.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum RequestOperationType {
    ReplaceFullPath,
    ReplacePrefixMatch,
}
/// RequestRedirect defines a schema for a filter that responds to the
/// request with an HTTP redirection.
///
/// Support: Core
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum RedirectStatusCode {
    #[serde(rename = "301")]
    r#_301,
    #[serde(rename = "302")]
    r#_302,
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
pub struct MatchingHeaders {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<HeaderMatchType>,
    pub value: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RequestMirror {
    #[serde(rename = "backendRef")]
    pub backend_ref: RequestMirrorRef,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ParentRouteStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(rename = "controllerName")]
    pub controller_name: String,
    #[serde(rename = "parentRef")]
    pub parent_ref: RouteRef,
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
pub struct HTTPRouteRequestRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<RequestRedirectPath>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<RequestRedirectScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCode")]
    pub status_code: Option<i64>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RouteStatus {
    pub parents: Vec<ParentRouteStatus>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HTTPRouteUrlRewrite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<RequestRedirectPath>,
}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GRPCRouteFilter {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extensionRef")]
    pub extension_ref: Option<GatewayInfrastructureParametersRef>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "requestHeaderModifier"
    )]
    pub request_header_modifier: Option<HeaderModifier>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestMirror")]
    pub request_mirror: Option<RequestMirror>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseHeaderModifier"
    )]
    pub response_header_modifier: Option<HeaderModifier>,
    #[serde(rename = "type")]
    pub r#type: GRPCFilterType,
}


// Next attempt 

