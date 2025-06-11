use super::common_types::*;
// WARNING: generated file - manual changes will be overriden

impl Default for GRPCFilterType {
    fn default() -> Self {
        GRPCFilterType::RequestHeaderModifier
    }
}

impl Default for HTTPFilterType {
    fn default() -> Self {
        HTTPFilterType::RequestHeaderModifier
    }
}

impl Default for RequestOperationType {
    fn default() -> Self {
        RequestOperationType::ReplaceFullPath
    }
}
