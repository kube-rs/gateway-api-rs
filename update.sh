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

set -eou pipefail

VERSION="v1.2.1"

STANDARD_APIS=(
    gatewayclasses
    gateways
    httproutes
    referencegrants
    grpcroutes
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

export APIS_DIR='gateway-api/src/apis'
rm -rf $APIS_DIR/standard/
rm -rf $APIS_DIR/experimental/

cat << EOF > $APIS_DIR/mod.rs
pub mod experimental;
pub mod standard;
EOF


mkdir -p $APIS_DIR/standard/
mkdir -p $APIS_DIR/experimental/


echo "// WARNING! generated file do not edit" > $APIS_DIR/standard/mod.rs

for API in "${STANDARD_APIS[@]}"
do
    echo "generating standard api ${API}"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/${VERSION}/config/crd/standard/gateway.networking.k8s.io_${API}.yaml" | kopium --schema=derived --derive=JsonSchema --derive=Default --derive=PartialEq --docs -f - > $APIS_DIR/standard/${API}.rs
    echo "pub mod ${API};" >> $APIS_DIR/standard/mod.rs
done

# Standard API enums that need a Default trait impl along with their respective default variant.
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

# Create a comma separated string out of $ENUMS.
ENUMS_WITH_DEFAULTS=$(printf ",%s" "${ENUMS[@]}")
ENUMS_WITH_DEFAULTS=${ENUMS_WITH_DEFAULTS:1}

# The task searches for $GATEWAY_API_ENUMS in the enviornment to get the enum names and their default variants.
GATEWAY_API_ENUMS=${ENUMS_WITH_DEFAULTS} cargo xtask gen_enum_defaults >> $APIS_DIR/standard/enum_defaults.rs
echo "mod enum_defaults;" >> $APIS_DIR/standard/mod.rs

GATEWAY_CLASS_CONDITION_CONSTANTS="GatewayClassConditionType=Accepted"
GATEWAY_CLASS_REASON_CONSTANTS="GatewayClassConditionReason=Accepted,InvalidParameters,Pending,Unsupported,Waiting"
GATEWAY_CONDITION_CONSTANTS="GatewayConditionType=Programmed,Accepted,Ready"
GATEWAY_REASON_CONSTANTS="GatewayConditionReason=Programmed,Invalid,NoResources,AddressNotAssigned,AddressNotUsable,Accepted,ListenersNotValid,Pending,UnsupportedAddress,InvalidParameters,Ready,ListenersNotReady"
LISTENER_CONDITION_CONSTANTS="ListenerConditionType=Conflicted,Accepted,ResolvedRefs,Programmed,Ready"
LISTENER_REASON_CONSTANTS="ListenerConditionReason=HostnameConflict,ProtocolConflict,NoConflicts,Accepted,PortUnavailable,UnsupportedProtocol,ResolvedRefs,InvalidCertificateRef,InvalidRouteKinds,RefNotPermitted,Programmed,Invalid,Pending,Ready"

GATEWAY_CLASS_CONDITION_CONSTANTS=${GATEWAY_CLASS_CONDITION_CONSTANTS} GATEWAY_CLASS_REASON_CONSTANTS=${GATEWAY_CLASS_REASON_CONSTANTS} \
    GATEWAY_CONDITION_CONSTANTS=${GATEWAY_CONDITION_CONSTANTS} GATEWAY_REASON_CONSTANTS=${GATEWAY_REASON_CONSTANTS} \
    LISTENER_CONDITION_CONSTANTS=${LISTENER_CONDITION_CONSTANTS} LISTENER_REASON_CONSTANTS=${LISTENER_REASON_CONSTANTS} \
    cargo xtask gen_condition_constants >> $APIS_DIR/standard/constants.rs
echo "pub mod constants;" >> $APIS_DIR/standard/mod.rs

echo "// WARNING! generated file do not edit" > $APIS_DIR/experimental/mod.rs

for API in "${EXPERIMENTAL_APIS[@]}"
do
    echo "generating experimental api $API"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/${VERSION}/config/crd/experimental/gateway.networking.k8s.io_${API}.yaml" | kopium --schema=derived --derive=JsonSchema --derive=Default --derive=PartialEq --docs -f - > $APIS_DIR/experimental/${API}.rs
    echo "pub mod ${API};" >> $APIS_DIR/experimental/mod.rs
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
GATEWAY_API_ENUMS=${ENUMS_WITH_DEFAULTS} cargo xtask gen_enum_defaults >> $APIS_DIR/experimental/enum_defaults.rs
echo "mod enum_defaults;" >> $APIS_DIR/experimental/mod.rs

# GatewayClass conditions vary between standard and experimental
GATEWAY_CLASS_CONDITION_CONSTANTS="${GATEWAY_CLASS_CONDITION_CONSTANTS},SupportedVersion"
GATEWAY_CLASS_REASON_CONSTANTS="${GATEWAY_CLASS_REASON_CONSTANTS},SupportedVersion,UnsupportedVersion"

GATEWAY_CLASS_CONDITION_CONSTANTS=${GATEWAY_CLASS_CONDITION_CONSTANTS} GATEWAY_CLASS_REASON_CONSTANTS=${GATEWAY_CLASS_REASON_CONSTANTS} \
    GATEWAY_CONDITION_CONSTANTS=${GATEWAY_CONDITION_CONSTANTS} GATEWAY_REASON_CONSTANTS=${GATEWAY_REASON_CONSTANTS} \
    LISTENER_CONDITION_CONSTANTS=${LISTENER_CONDITION_CONSTANTS} LISTENER_REASON_CONSTANTS=${LISTENER_REASON_CONSTANTS} \
    cargo xtask gen_condition_constants >> $APIS_DIR/experimental/constants.rs
echo "pub mod constants;" >> $APIS_DIR/experimental/mod.rs

# Format the code.
cargo fmt


rm -rf $APIS_DIR/processed
mkdir -p  $APIS_DIR/processed
export RUST_LOG=debug

echo " **** PHASE 1 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/standard --out-dir $APIS_DIR/processed --current-pass-substitute-names ./type-reducer/customized_mapped_names_pass_1_with_enums.txt
mv mapped_names.txt mapped_names_phase_1.txt
mv mapped_types_to_names.txt mapped_types_to_names_pahse_1.txt
echo " **** PHASE 2 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/processed --out-dir $APIS_DIR/processed --previous-pass-derived-type-names ./type-reducer/reduced_types_pass_1_with_enums.txt --current-pass-substitute-names ./type-reducer/customized_mapped_names_pass_2_with_enums.txt
mv mapped_names.txt mapped_names_phase_2.txt
mv mapped_types_to_names.txt mapped_types_to_names_pahse_2.txt
echo " **** PHASE 3 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/processed --out-dir $APIS_DIR/processed --previous-pass-derived-type-names ./type-reducer/reduced_types_pass_2_with_enums.txt --current-pass-substitute-names ./type-reducer/customized_mapped_names_pass_3_with_enums.txt
mv mapped_names.txt mapped_names_phase_3.txt
mv mapped_types_to_names.txt mapped_types_to_names_pahse_3.txt
echo " **** PHASE 4 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/processed --out-dir $APIS_DIR/processed --previous-pass-derived-type-names ./type-reducer/reduced_types_pass_3_with_enums.txt --current-pass-substitute-names ./type-reducer/customized_mapped_names_pass_4_with_enums.txt
mv mapped_names.txt mapped_names_phase_4.txt
mv mapped_types_to_names.txt mapped_types_to_names_pahse_4.txt

cat << EOF >> $APIS_DIR/mod.rs

pub mod processed;
EOF

ENUMS=(
    GRPCFilterType=RequestHeaderModifier
    HTTPPathType=ReplaceFullPath
    HTTPFilterType=RequestHeaderModifier
)

ENUMS_WITH_DEFAULTS=$(printf ",%s" "${ENUMS[@]}")
ENUMS_WITH_DEFAULTS=${ENUMS_WITH_DEFAULTS:1}
echo "use super::common_types::*;" > $APIS_DIR/processed/enum_defaults.rs
GATEWAY_API_ENUMS=${ENUMS_WITH_DEFAULTS} cargo xtask gen_enum_defaults >> $APIS_DIR/processed/enum_defaults.rs




