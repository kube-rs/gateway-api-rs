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

VERSION="v1.4.0"

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
    sed -i 's/pub use kube::CustomResource;/pub use kube_derive::CustomResource;/g' $APIS_DIR/standard/${API}.rs
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
ROUTE_CONDITION_CONSTANTS="RouteConditionType=Accepted,ResolvedRefs,PartiallyInvalid"
ROUTE_REASON_CONSTANTS="RouteConditionReason=Accepted,NotAllowedByListeners,NoMatchingListenerHostname,NoMatchingParent,UnsupportedValue,Pending,IncompatibleFilters,ResolvedRefs,RefNotPermitted,InvalidKind,BackendNotFound,UnsupportedProtocol"

GATEWAY_CLASS_CONDITION_CONSTANTS=${GATEWAY_CLASS_CONDITION_CONSTANTS} GATEWAY_CLASS_REASON_CONSTANTS=${GATEWAY_CLASS_REASON_CONSTANTS} \
    GATEWAY_CONDITION_CONSTANTS=${GATEWAY_CONDITION_CONSTANTS} GATEWAY_REASON_CONSTANTS=${GATEWAY_REASON_CONSTANTS} \
    LISTENER_CONDITION_CONSTANTS=${LISTENER_CONDITION_CONSTANTS} LISTENER_REASON_CONSTANTS=${LISTENER_REASON_CONSTANTS} \
    ROUTE_CONDITION_CONSTANTS=${ROUTE_CONDITION_CONSTANTS} ROUTE_REASON_CONSTANTS=${ROUTE_REASON_CONSTANTS} \
    cargo xtask gen_condition_constants >> $APIS_DIR/standard/constants.rs
echo "pub mod constants;" >> $APIS_DIR/standard/mod.rs

echo "// WARNING! generated file do not edit" > $APIS_DIR/experimental/mod.rs

for API in "${EXPERIMENTAL_APIS[@]}"
do
    echo "generating experimental api $API"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/${VERSION}/config/crd/experimental/gateway.networking.k8s.io_${API}.yaml" | kopium --schema=derived --derive=JsonSchema --derive=Default --derive=PartialEq --docs -f - > $APIS_DIR/experimental/${API}.rs
    sed -i 's/pub use kube::CustomResource;/pub use kube_derive::CustomResource;/g' $APIS_DIR/experimental/${API}.rs
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
    HTTPRouteRulesBackendRefsFiltersExternalAuthProtocol=HTTP
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
ROUTE_CONDITION_CONSTANTS="RouteConditionType=Accepted,ResolvedRefs"
ROUTE_REASON_CONSTANTS="RouteConditionReason=Accepted,NotAllowedByListeners,NoMatchingListenerHostname,UnsupportedValue,Pending,ResolvedRefs,RefNotPermitted,InvalidKind,BackendNotFound"

GATEWAY_CLASS_CONDITION_CONSTANTS=${GATEWAY_CLASS_CONDITION_CONSTANTS} GATEWAY_CLASS_REASON_CONSTANTS=${GATEWAY_CLASS_REASON_CONSTANTS} \
    GATEWAY_CONDITION_CONSTANTS=${GATEWAY_CONDITION_CONSTANTS} GATEWAY_REASON_CONSTANTS=${GATEWAY_REASON_CONSTANTS} \
    LISTENER_CONDITION_CONSTANTS=${LISTENER_CONDITION_CONSTANTS} LISTENER_REASON_CONSTANTS=${LISTENER_REASON_CONSTANTS} \
    ROUTE_CONDITION_CONSTANTS=${ROUTE_CONDITION_CONSTANTS} ROUTE_REASON_CONSTANTS=${ROUTE_REASON_CONSTANTS} \
    cargo xtask gen_condition_constants >> $APIS_DIR/experimental/constants.rs
echo "pub mod constants;" >> $APIS_DIR/experimental/mod.rs

# Format the code.
cargo fmt


export RUST_LOG=info

echo " **** Starting Type Reducer - Collapsing Duplicative Types **** "
echo " **** Type Reducer - PHASE 1 - First Pass ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/standard --out-dir $APIS_DIR/standard reduce --previous-pass-derived-type-names ./type-reducer/standard_reduced_types_pass_0.txt --current-pass-substitute-names ./type-reducer/standard_customized_mapped_names.txt
mv mapped_names.txt standard_mapped_names_phase_1.txt
mv mapped_types_to_names.txt standard_mapped_types_to_names_phase_1.txt
echo " **** PHASE 2 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/standard --out-dir $APIS_DIR/standard reduce --previous-pass-derived-type-names ./type-reducer/standard_reduced_types_pass_1.txt --current-pass-substitute-names ./type-reducer/standard_customized_mapped_names.txt
mv mapped_names.txt standard_mapped_names_phase_2.txt
mv mapped_types_to_names.txt standard_mapped_types_to_names_phase_2.txt
echo " **** PHASE 3 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/standard --out-dir $APIS_DIR/standard reduce --previous-pass-derived-type-names ./type-reducer/standard_reduced_types_pass_2.txt --current-pass-substitute-names ./type-reducer/standard_customized_mapped_names.txt
mv mapped_names.txt standard_mapped_names_phase_3.txt
mv mapped_types_to_names.txt standard_mapped_types_to_names_phase_3.txt

echo " **** RENAMING PHASE ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/standard --out-dir $APIS_DIR/standard rename --rename-only-substitute-names ./type-reducer/standard_rename_only_mapped_names.txt


ENUMS=(
    GRPCFilterType=RequestHeaderModifier
    RequestOperationType=ReplaceFullPath
    HTTPFilterType=RequestHeaderModifier    
)

ENUMS_WITH_DEFAULTS=$(printf ",%s" "${ENUMS[@]}")
ENUMS_WITH_DEFAULTS=${ENUMS_WITH_DEFAULTS:1}
echo "use super::common::*;" > $APIS_DIR/standard/enum_defaults.rs
GATEWAY_API_ENUMS=${ENUMS_WITH_DEFAULTS} cargo xtask gen_enum_defaults >> $APIS_DIR/standard/enum_defaults.rs

sed -i '/#\[kube(status = "GRPCRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/standard/grpcroutes.rs
sed -i '/#\[kube(status = "HTTPRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/standard/httproutes.rs

export RUST_LOG=info
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/experimental --out-dir $APIS_DIR/experimental reduce --previous-pass-derived-type-names ./type-reducer/experimental_reduced_types_pass_0.txt --current-pass-substitute-names ./type-reducer/experimental_customized_mapped_names.txt
mv mapped_names.txt experimental_mapped_names_phase_1.txt
mv mapped_types_to_names.txt experimental_mapped_types_to_names_phase_1.txt
echo " **** PHASE 2 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/experimental --out-dir $APIS_DIR/experimental reduce --previous-pass-derived-type-names ./type-reducer/experimental_reduced_types_pass_1.txt --current-pass-substitute-names ./type-reducer/experimental_customized_mapped_names.txt
mv mapped_names.txt experimental_mapped_names_phase_2.txt
mv mapped_types_to_names.txt experimental_mapped_types_to_names_phase_2.txt
echo " **** PHASE 3 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/experimental --out-dir $APIS_DIR/experimental reduce --previous-pass-derived-type-names ./type-reducer/experimental_reduced_types_pass_2.txt --current-pass-substitute-names ./type-reducer/experimental_customized_mapped_names.txt --ignorable-type-names ./type-reducer/experimental_ignorable_mapped_names.txt
mv mapped_names.txt experimental_mapped_names_phase_3.txt
mv mapped_types_to_names.txt experimental_mapped_types_to_names_phase_3.txt
echo " **** PHASE 4 ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/experimental --out-dir $APIS_DIR/experimental reduce --previous-pass-derived-type-names ./type-reducer/experimental_reduced_types_pass_3.txt --current-pass-substitute-names ./type-reducer/experimental_customized_mapped_names.txt --ignorable-type-names ./type-reducer/experimental_ignorable_mapped_names.txt
mv mapped_names.txt experimental_mapped_names_phase_4.txt
mv mapped_types_to_names.txt experimental_mapped_types_to_names_phase_4.txt

echo " **** RENAMING PHASE ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $APIS_DIR/experimental --out-dir $APIS_DIR/experimental rename --rename-only-substitute-names ./type-reducer/experimental_rename_only_mapped_names.txt

ENUMS=(
    GRPCFilterType=RequestHeaderModifier
    RequestOperationType=ReplaceFullPath
    HTTPFilterType=RequestHeaderModifier    
)

ENUMS_WITH_DEFAULTS=$(printf ",%s" "${ENUMS[@]}")
ENUMS_WITH_DEFAULTS=${ENUMS_WITH_DEFAULTS:1}
echo "use super::common::*;" > $APIS_DIR/experimental/enum_defaults.rs
GATEWAY_API_ENUMS=${ENUMS_WITH_DEFAULTS} cargo xtask gen_enum_defaults >> $APIS_DIR/experimental/enum_defaults.rs

sed -i '/#\[kube(status = "GRPCRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/grpcroutes.rs
sed -i '/#\[kube(status = "HTTPRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/httproutes.rs
sed -i '/#\[kube(status = "TLSRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/tlsroutes.rs
sed -i '/#\[kube(status = "UDPRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/udproutes.rs
sed -i '/#\[kube(status = "TCPRouteStatus")\]/c\#\[kube(status = "RouteStatus")\]' $APIS_DIR/experimental/tcproutes.rs

cargo fmt
