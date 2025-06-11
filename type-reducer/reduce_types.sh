#!/bin/bash
rm -rf ../gateway-api/src/apis/processed
mkdir ../gateway-api/src/apis/processed
#cargo run -- --apis-dir ../gateway-api/src/apis/standard --out-dir ../gateway-api/src/apis/processed
echo "\n\n PHASE 1\n\n"
cargo run -- --apis-dir ../gateway-api/src/apis/standard --out-dir ../gateway-api/src/apis/processed --current-pass-substitute-names customized_mapped_names.txt --previous-pass-derived-type-names reduced_types_pass_0.txt
echo "\n\n PHASE 2\n\n"
cargo run -- --apis-dir ../gateway-api/src/apis/processed --out-dir ../gateway-api/src/apis/processed --previous-pass-derived-type-names reduced_types_pass_1.txt --current-pass-substitute-names customized_mapped_names.txt

