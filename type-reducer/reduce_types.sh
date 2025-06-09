#!/bin/bash
rm -rf ../gateway-api/src/apis/processed
mkdir ../gateway-api/src/apis/processed
cargo run -- --apis-dir ../gateway-api/src/apis/standard --out-dir ../gateway-api/src/apis/processed
#cargo run -- --apis-dir ../gateway-api/src/apis/standard --out-dir ../gateway-api/src/apis/processed --current-pass-substitute-names customized_mapped_names_pass_1_with_enums.txt
#cargo run -- --apis-dir ../gateway-api/src/apis/processed --out-dir ../gateway-api/src/apis/processed --previous-pass-derived-type-names reduced_types_pass_1_with_enums.txt --current-pass-substitute-names customized_mapped_names_pass_2_with_enums.txt


