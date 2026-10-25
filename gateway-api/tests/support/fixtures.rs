use std::{collections::BTreeMap, fs, path::Path};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;

use super::Channel;

/// The vendored upstream CRDs of `channel`, by resource, e.g. `httproutes`:
/// the same YAML the bindings are generated from (see `cargo xtask generate`),
/// in fixtures named `<group>_<resource>.yaml`.
pub fn upstream_crds(channel: Channel) -> BTreeMap<String, CustomResourceDefinition> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/crds")
        .join(channel.name());
    let entries = fs::read_dir(&dir).unwrap_or_else(|e| panic!("failed to read {dir}: {e}", dir = dir.display()));
    let mut crds = BTreeMap::new();
    for entry in entries {
        let path = entry.expect("failed to read a directory entry").path();
        let resource = path
            .file_stem()
            .and_then(|stem| stem.to_str()?.split_once('_'))
            .unwrap_or_else(|| panic!("{path} isn't named <group>_<resource>.yaml", path = path.display()))
            .1;
        let yaml =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}", path = path.display()));
        let crd = serde_saphyr::from_str(&yaml)
            .unwrap_or_else(|e| panic!("failed to parse {path}: {e}", path = path.display()));
        crds.insert(resource.to_owned(), crd);
    }
    crds
}
