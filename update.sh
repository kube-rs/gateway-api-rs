#!/bin/bash

set -eoux pipefail

VERSION="v0.5.1"

STANDARD_APIS=(
    gatewayclasses
    gateways
    httproutes # TODO: not working
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
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/main/config/crd/standard/gateway.networking.k8s.io_${API}.yaml?ref=${VERSION}" | kopium -Af - > src/apis/standard/${API}.rs
    echo "pub mod ${API};" >> src/apis/standard/mod.rs
done

echo "// WARNING! generated file do not edit" > src/apis/experimental/mod.rs

for API in "${EXPERIMENTAL_APIS[@]}"
do
    echo "generating experimental api $API"
    curl -sSL "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api/main/config/crd/experimental/gateway.networking.k8s.io_${API}.yaml?ref=${VERSION}" | kopium -Af - > src/apis/experimental/${API}.rs
    echo "pub mod ${API};" >> src/apis/experimental/mod.rs
done
