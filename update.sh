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

VERSION="v1.0.0"

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

rm -rf src/apis/

mkdir -p src/apis/
cat << EOF > src/apis/mod.rs
pub mod experimental;
pub mod standard;
EOF

mkdir -p src/apis/standard/
mkdir -p src/apis/experimental/

echo "// WARNING! generated file do not edit" > src/apis/standard/mod.rs

for API in "${STANDARD_APIS[@]}"
do
    echo "generating standard api ${API}"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/${VERSION}/config/crd/standard/gateway.networking.k8s.io_${API}.yaml" | kopium -Af - > src/apis/standard/${API}.rs
    echo "pub mod ${API};" >> src/apis/standard/mod.rs
done

echo "// WARNING! generated file do not edit" > src/apis/experimental/mod.rs

for API in "${EXPERIMENTAL_APIS[@]}"
do
    echo "generating experimental api $API"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/${VERSION}/config/crd/experimental/gateway.networking.k8s.io_${API}.yaml" | kopium -Af - > src/apis/experimental/${API}.rs
    echo "pub mod ${API};" >> src/apis/experimental/mod.rs
done

cargo fmt
