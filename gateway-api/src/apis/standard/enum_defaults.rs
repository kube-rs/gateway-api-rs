// WARNING: generated file — do not edit

use super::backendtlspolicies::BackendTlsPolicyValidationSubjectAltNamesType;

use super::grpcroutes::{GrpcRouteRulesBackendRefsFiltersType, GrpcRouteRulesFiltersType};

use super::httproutes::{
    HttpRouteRulesBackendRefsFiltersRequestRedirectPathType, HttpRouteRulesBackendRefsFiltersType,
    HttpRouteRulesBackendRefsFiltersUrlRewritePathType,
    HttpRouteRulesFiltersRequestRedirectPathType, HttpRouteRulesFiltersType,
    HttpRouteRulesFiltersUrlRewritePathType,
};

impl Default for BackendTlsPolicyValidationSubjectAltNamesType {
    fn default() -> Self {
        BackendTlsPolicyValidationSubjectAltNamesType::Hostname
    }
}

impl Default for GrpcRouteRulesBackendRefsFiltersType {
    fn default() -> Self {
        GrpcRouteRulesBackendRefsFiltersType::RequestHeaderModifier
    }
}

impl Default for GrpcRouteRulesFiltersType {
    fn default() -> Self {
        GrpcRouteRulesFiltersType::RequestHeaderModifier
    }
}

impl Default for HttpRouteRulesBackendRefsFiltersRequestRedirectPathType {
    fn default() -> Self {
        HttpRouteRulesBackendRefsFiltersRequestRedirectPathType::ReplaceFullPath
    }
}

impl Default for HttpRouteRulesBackendRefsFiltersType {
    fn default() -> Self {
        HttpRouteRulesBackendRefsFiltersType::RequestHeaderModifier
    }
}

impl Default for HttpRouteRulesBackendRefsFiltersUrlRewritePathType {
    fn default() -> Self {
        HttpRouteRulesBackendRefsFiltersUrlRewritePathType::ReplaceFullPath
    }
}

impl Default for HttpRouteRulesFiltersRequestRedirectPathType {
    fn default() -> Self {
        HttpRouteRulesFiltersRequestRedirectPathType::ReplaceFullPath
    }
}

impl Default for HttpRouteRulesFiltersType {
    fn default() -> Self {
        HttpRouteRulesFiltersType::RequestHeaderModifier
    }
}

impl Default for HttpRouteRulesFiltersUrlRewritePathType {
    fn default() -> Self {
        HttpRouteRulesFiltersUrlRewritePathType::ReplaceFullPath
    }
}
