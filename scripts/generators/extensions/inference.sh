#!/bin/bash

# ------------------------------------------------------------------------------
# This script will automatically generate API updates for new Inference Extension
# releases. Update the $INFERENCE_EXT_VERSION to the new release version before
# executing.
#
# This script requires kopium, which can be installed with:
#
#   cargo install kopium
#
# See: https://github.com/kube-rs/kopium
# ------------------------------------------------------------------------------
set -euo pipefail

export EXTENSION_DIR=extensions
echo " **** Inference Extension Processing Starts **** "

INFERENCE_EXT_VERSION="v1.0.2"
INFERENCE_API_DIR=${EXTENSION_DIR}/inference/src/apis
echo "Using Inference Extension version ${INFERENCE_EXT_VERSION}"

INFERENCE_EXT_STANDARD_APIS=(
    inferencepools
)

INFERENCE_EXT_EXPERIMENTAL_APIS=(
    inferencepools
    inferenceobjectives
)

rm -rf $INFERENCE_API_DIR/standard
rm -rf $INFERENCE_API_DIR/experimental

mkdir -p $INFERENCE_API_DIR/standard
mkdir -p $INFERENCE_API_DIR/experimental


echo "// WARNING! generated file do not edit" > $INFERENCE_API_DIR/standard/mod.rs

for API in "${INFERENCE_EXT_STANDARD_APIS[@]}"
do
    echo "generating inference extension standard api ${API}"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api-inference-extension/${INFERENCE_EXT_VERSION}/config/crd/bases/inference.networking.k8s.io_${API}.yaml" | kopium --schema=derived --derive=JsonSchema --derive=Default --derive=PartialEq --docs -f - > $INFERENCE_API_DIR/standard/${API}.rs
    sed -i 's/pub use kube::CustomResource;/pub use kube_derive::CustomResource;/g' $INFERENCE_API_DIR/standard/${API}.rs
    echo "pub mod ${API};" >> $INFERENCE_API_DIR/standard/mod.rs
done

echo "// WARNING! generated file do not edit" > $INFERENCE_API_DIR/experimental/mod.rs

for API in "${INFERENCE_EXT_EXPERIMENTAL_APIS[@]}"
do
    echo "generating inference extension experimental api ${API}"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api-inference-extension/${INFERENCE_EXT_VERSION}/config/crd/bases/inference.networking.x-k8s.io_${API}.yaml" | kopium --schema=derived --derive=JsonSchema --derive=Default --derive=PartialEq --docs -f - > $INFERENCE_API_DIR/experimental/${API}.rs
    sed -i 's/pub use kube::CustomResource;/pub use kube_derive::CustomResource;/g' $INFERENCE_API_DIR/experimental/${API}.rs
    echo "pub mod ${API};" >> $INFERENCE_API_DIR/experimental/mod.rs
done


export RUST_LOG=info

export TMP_ARTIFACTS="artifacts"
mkdir -p ${TMP_ARTIFACTS}/inference/

echo " **** Standard APIs Start **** "
echo " **** Starting Type Reducer - Collapsing Duplicative Types **** "
echo " **** Type Reducer - PHASE 1 - First Pass ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $INFERENCE_API_DIR/standard --out-dir $INFERENCE_API_DIR/standard reduce --previous-pass-derived-type-names ./type-reducer/extension/inference/standard_reduced_types_pass_0.txt --current-pass-substitute-names ./type-reducer/extension/inference/standard_customized_mapped_names.txt
mv mapped_names.txt ${TMP_ARTIFACTS}/inference/standard_extension_inference_mapped_names_phase_1.txt
mv mapped_types_to_names.txt ${TMP_ARTIFACTS}/inference/standard_extension_inference_mapped_types_to_names_phase_1.txt

echo " **** RENAMING PHASE ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $INFERENCE_API_DIR/standard --out-dir $INFERENCE_API_DIR/standard rename --rename-only-substitute-names ./type-reducer/extension/inference/standard_rename_only_mapped_names.txt
echo " **** Standard APIs End **** "


echo " **** Experimental APIs Start **** "
echo " **** Starting Type Reducer - Collapsing Duplicative Types **** "
echo " **** Type Reducer - PHASE 1 - First Pass ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $INFERENCE_API_DIR/experimental --out-dir $INFERENCE_API_DIR/experimental reduce --previous-pass-derived-type-names ./type-reducer/extension/inference/experimental_reduced_types_pass_0.txt --current-pass-substitute-names ./type-reducer/extension/inference/experimental_customized_mapped_names.txt
mv mapped_names.txt ${TMP_ARTIFACTS}/inference/experimental_extension_inference_mapped_names_phase_1.txt
mv mapped_types_to_names.txt ${TMP_ARTIFACTS}/inference/experimental_extension_inference_mapped_types_to_names_phase_1.txt

echo " **** RENAMING PHASE ***** "
cargo run --manifest-path type-reducer/Cargo.toml -- --apis-dir $INFERENCE_API_DIR/experimental --out-dir $INFERENCE_API_DIR/experimental rename --rename-only-substitute-names ./type-reducer/extension/inference/experimental_rename_only_mapped_names.txt
echo " **** Experimental APIs End **** "


cargo fmt
echo "Inference Extension API Generation complete"

echo "Inference Extension API Cleaning up temporary files"
set -x
rm ${TMP_ARTIFACTS}/inference/standard_extension_inference_mapped_names_phase_*.txt
rm ${TMP_ARTIFACTS}/inference/standard_extension_inference_mapped_types_to_names_phase_*.txt
rm ${TMP_ARTIFACTS}/inference/experimental_extension_inference_mapped_names_phase_*.txt
rm ${TMP_ARTIFACTS}/inference/experimental_extension_inference_mapped_types_to_names_phase_*.txt
set +x
echo "Inference Extension API Cleanup complete"