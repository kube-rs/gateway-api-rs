#!/bin/bash

# ------------------------------------------------------------------------------
# This script will automatically generate API updates for new Gateway API
# releases. Update the $VERSION to the new release version before executing.
#
# This script requires kopium, which can be installed with:
#
#   cargo install kopium
#
# See: https://github.com/kube-rs/kopium
# ------------------------------------------------------------------------------

set -eoux pipefail

VERSION="v1.1.0"

STANDARD_APIS=(
    gatewayclasses
    gateways
    httproutes
    referencegrants
)

EXPERIMENTAL_APIS=(
    gatewayclasses
    gateways
    httproutes
    referencegrants
    grpcroutes
    tcproutes
    tlsroutes
    udproutes
)

rm -rf gateway-api/src/apis/

mkdir -p gateway-api/src/apis/
cat << EOF > gateway-api/src/apis/mod.rs
pub mod experimental;
pub mod standard;
EOF

mkdir -p gateway-api/src/apis/standard/
mkdir -p gateway-api/src/apis/experimental/

echo "// WARNING! generated file do not edit" > gateway-api/src/apis/standard/mod.rs

for API in "${STANDARD_APIS[@]}"
do
    echo "generating standard api ${API}"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/${VERSION}/config/crd/standard/gateway.networking.k8s.io_${API}.yaml" | kopium --schema=derived --derive=JsonSchema --derive=Default --docs -f - > gateway-api/src/apis/standard/${API}.rs
    echo "pub mod ${API};" >> gateway-api/src/apis/standard/mod.rs
done

# Standard API enums that need a Default trait impl along with their respective default variant.
ENUMS=(
    HTTPRouteRulesFiltersRequestRedirectPathType=ReplaceFullPath
    HTTPRouteRulesFiltersUrlRewritePathType=ReplaceFullPath
    HTTPRouteRulesFiltersType=RequestHeaderModifier
    HTTPRouteRulesBackendRefsFiltersRequestRedirectPathType=ReplaceFullPath
    HTTPRouteRulesBackendRefsFiltersUrlRewritePathType=ReplaceFullPath
    HTTPRouteRulesBackendRefsFiltersType=RequestHeaderModifier
)

# Create a comma separated string out of $ENUMS.
ENUMS_WITH_DEFAULTS=$(printf ",%s" "${ENUMS[@]}")
ENUMS_WITH_DEFAULTS=${ENUMS_WITH_DEFAULTS:1}

# The task searches for $GATEWAY_API_ENUMS in the enviornment to get the enum names and their default variants.
GATEWAY_API_ENUMS=${ENUMS_WITH_DEFAULTS} cargo xtask gen_enum_defaults >> gateway-api/src/apis/standard/enum_defaults.rs
echo "mod enum_defaults;" >> gateway-api/src/apis/standard/mod.rs

GATEWAY_CONDITION_CONSTANTS="GatewayConditionType=Programmed,Accepted,Ready"
GATEWAY_REASON_CONSTANTS="GatewayConditionReason=Programmed,Invalid,NoResources,AddressNotAssigned,AddressNotUsable,Accepted,ListenersNotValid,Pending,UnsupportedAddress,InvalidParameters,Ready,ListenersNotReady"
LISTENER_CONDITION_CONSTANTS="ListenerConditionType=Conflicted,Accepted,ResolvedRefs,Programmed,Ready"
LISTENER_REASON_CONSTANTS="ListenerConditionReason=HostnameConflict,ProtocolConflict,NoConflicts,Accepted,PortUnavailable,UnsupportedProtocol,ResolvedRefs,InvalidCertificateRef,InvalidRouteKinds,RefNotPermitted,Programmed,Invalid,Pending,Ready"

GATEWAY_CONDITION_CONSTANTS=${GATEWAY_CONDITION_CONSTANTS} GATEWAY_REASON_CONSTANTS=${GATEWAY_REASON_CONSTANTS} \
    LISTENER_CONDITION_CONSTANTS=${LISTENER_CONDITION_CONSTANTS} LISTENER_REASON_CONSTANTS=${LISTENER_REASON_CONSTANTS} \
    cargo xtask gen_condition_constants >> gateway-api/src/apis/standard/constants.rs
echo "pub mod constants;" >> gateway-api/src/apis/standard/mod.rs

echo "// WARNING! generated file do not edit" > gateway-api/src/apis/experimental/mod.rs

for API in "${EXPERIMENTAL_APIS[@]}"
do
    echo "generating experimental api $API"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/${VERSION}/config/crd/experimental/gateway.networking.k8s.io_${API}.yaml" | kopium --schema=derived --derive=JsonSchema --derive=Default --docs -f - > gateway-api/src/apis/experimental/${API}.rs
    echo "pub mod ${API};" >> gateway-api/src/apis/experimental/mod.rs
done

# Experimental API enums that need a Default trait impl along with their respective default variant.
ENUMS=(
    HTTPRouteRulesFiltersRequestRedirectPathType=ReplaceFullPath
    HTTPRouteRulesFiltersUrlRewritePathType=ReplaceFullPath
    HTTPRouteRulesFiltersType=RequestHeaderModifier
    HTTPRouteRulesBackendRefsFiltersRequestRedirectPathType=ReplaceFullPath
    HTTPRouteRulesBackendRefsFiltersUrlRewritePathType=ReplaceFullPath
    HTTPRouteRulesBackendRefsFiltersType=RequestHeaderModifier
    GRPCRouteRulesFiltersType=RequestHeaderModifier
    GRPCRouteRulesBackendRefsFiltersType=RequestHeaderModifier
)

ENUMS_WITH_DEFAULTS=$(printf ",%s" "${ENUMS[@]}")
ENUMS_WITH_DEFAULTS=${ENUMS_WITH_DEFAULTS:1}
GATEWAY_API_ENUMS=${ENUMS_WITH_DEFAULTS} cargo xtask gen_enum_defaults >> gateway-api/src/apis/experimental/enum_defaults.rs
echo "mod enum_defaults;" >> gateway-api/src/apis/experimental/mod.rs

GATEWAY_CONDITION_CONSTANTS=${GATEWAY_CONDITION_CONSTANTS} GATEWAY_REASON_CONSTANTS=${GATEWAY_REASON_CONSTANTS} \
    LISTENER_CONDITION_CONSTANTS=${LISTENER_CONDITION_CONSTANTS} LISTENER_REASON_CONSTANTS=${LISTENER_REASON_CONSTANTS} \
    cargo xtask gen_condition_constants >> gateway-api/src/apis/experimental/constants.rs
echo "pub mod constants;" >> gateway-api/src/apis/experimental/mod.rs

# Format the code.
cargo fmt
