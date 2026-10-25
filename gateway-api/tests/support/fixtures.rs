use std::{collections::BTreeMap, fs, path::Path};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;

use super::Channel;

/// The vendored upstream CRDs of `channel`, by resource (the CRD's plural
/// name, e.g. `httproutes`): the same YAML the bindings are generated from
/// (see `cargo xtask generate`).
pub fn upstream_crds(channel: Channel) -> BTreeMap<String, CustomResourceDefinition> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/crds")
        .join(channel.name());
    let entries = fs::read_dir(&dir).unwrap_or_else(|e| panic!("failed to read {dir}: {e}", dir = dir.display()));
    let mut crds = BTreeMap::new();
    for entry in entries {
        let path = entry.expect("failed to read a directory entry").path();
        let yaml =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}", path = path.display()));
        let crd: CustomResourceDefinition = serde_saphyr::from_str(&yaml)
            .unwrap_or_else(|e| panic!("failed to parse {path}: {e}", path = path.display()));
        let resource = crd.spec.names.plural.clone();
        if crds.insert(resource.clone(), crd).is_some() {
            panic!("{channel} has more than one CRD for {resource}");
        }
    }
    crds
}
