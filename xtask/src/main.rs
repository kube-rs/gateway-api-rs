#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

use std::{
    collections::BTreeSet,
    env,
    fmt::Write,
    fs,
    path::Path,
    process::{self, Command, Stdio},
};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

const APIS_DIR: &str = "gateway-api/src/apis";

const CRD_BASE_URL: &str = "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api";

const CRD_PATH: &str = "config/crd";

const CRD_PREFIX_STANDARD: &str = "gateway.networking.k8s.io_";
const CRD_PREFIX_EXPERIMENTAL: &str = "gateway.networking.x-k8s.io_";

const KOPIUM_ARGS: &[&str] = &[
    "--schema=derived",
    "--derive=JsonSchema",
    "--derive=Default",
    "--derive=PartialEq",
    "--docs",
    "-f",
    "-",
];

const GENERATED_HEADER: &str = "// WARNING: generated file — do not edit\n";

// -----------------------------------------------------------------------------
// API channels
// -----------------------------------------------------------------------------

struct Api {
    name: &'static str,
    crd_prefix: &'static str,
    /// API version re-exported at the resource module root.
    root_version: &'static str,
}

impl Api {
    const fn standard(name: &'static str, root_version: &'static str) -> Self {
        Self {
            name,
            crd_prefix: CRD_PREFIX_STANDARD,
            root_version,
        }
    }

    const fn experimental(name: &'static str, root_version: &'static str) -> Self {
        Self {
            name,
            crd_prefix: CRD_PREFIX_EXPERIMENTAL,
            root_version,
        }
    }
}

struct Channel {
    name: &'static str,
    apis: &'static [Api],
    enum_defaults: &'static [(&'static str, &'static str)],
    conditions: &'static [(&'static str, &'static [&'static str])],
}

const STANDARD: Channel = Channel {
    name: "standard",
    apis: &[
        Api::standard("backendtlspolicies", "v1"),
        Api::standard("gatewayclasses", "v1"),
        Api::standard("gateways", "v1"),
        Api::standard("grpcroutes", "v1"),
        Api::standard("httproutes", "v1"),
        Api::standard("listenersets", "v1"),
        Api::standard("referencegrants", "v1"),
        Api::standard("tlsroutes", "v1"),
    ],
    enum_defaults: &[
        ("BackendTlsPolicyValidationSubjectAltNamesType", "Hostname"),
        ("GrpcRouteRulesBackendRefsFiltersType", "RequestHeaderModifier"),
        ("GrpcRouteRulesFiltersType", "RequestHeaderModifier"),
        (
            "HttpRouteRulesBackendRefsFiltersRequestRedirectPathType",
            "ReplaceFullPath",
        ),
        ("HttpRouteRulesBackendRefsFiltersType", "RequestHeaderModifier"),
        ("HttpRouteRulesBackendRefsFiltersUrlRewritePathType", "ReplaceFullPath"),
        ("HttpRouteRulesFiltersRequestRedirectPathType", "ReplaceFullPath"),
        ("HttpRouteRulesFiltersType", "RequestHeaderModifier"),
        ("HttpRouteRulesFiltersUrlRewritePathType", "ReplaceFullPath"),
    ],
    conditions: &[
        ("GatewayClassConditionType", &["Accepted"]),
        (
            "GatewayClassConditionReason",
            &["Accepted", "InvalidParameters", "Pending", "Unsupported", "Waiting"],
        ),
        ("GatewayConditionType", &["Programmed", "Accepted", "Ready"]),
        (
            "GatewayConditionReason",
            &[
                "Programmed",
                "Invalid",
                "NoResources",
                "AddressNotAssigned",
                "AddressNotUsable",
                "Accepted",
                "ListenersNotValid",
                "Pending",
                "UnsupportedAddress",
                "InvalidParameters",
                "Ready",
                "ListenersNotReady",
            ],
        ),
        (
            "ListenerConditionType",
            &["Conflicted", "Accepted", "ResolvedRefs", "Programmed", "Ready"],
        ),
        (
            "ListenerConditionReason",
            &[
                "HostnameConflict",
                "ProtocolConflict",
                "NoConflicts",
                "Accepted",
                "PortUnavailable",
                "UnsupportedProtocol",
                "ResolvedRefs",
                "InvalidCertificateRef",
                "InvalidRouteKinds",
                "RefNotPermitted",
                "Programmed",
                "Invalid",
                "Pending",
                "Ready",
            ],
        ),
    ],
};

const EXPERIMENTAL: Channel = Channel {
    name: "experimental",
    apis: &[
        Api::standard("backendtlspolicies", "v1"),
        Api::standard("gatewayclasses", "v1"),
        Api::standard("gateways", "v1"),
        Api::standard("grpcroutes", "v1"),
        Api::standard("httproutes", "v1"),
        Api::standard("listenersets", "v1"),
        Api::standard("referencegrants", "v1"),
        Api::standard("tcproutes", "v1alpha2"),
        Api::standard("tlsroutes", "v1"),
        Api::standard("udproutes", "v1alpha2"),
        Api::experimental("xbackendtrafficpolicies", "v1alpha1"),
        Api::experimental("xmeshes", "v1alpha1"),
    ],
    enum_defaults: &[
        ("BackendTlsPolicyValidationSubjectAltNamesType", "Hostname"),
        ("GrpcRouteRulesBackendRefsFiltersType", "RequestHeaderModifier"),
        ("GrpcRouteRulesFiltersType", "RequestHeaderModifier"),
        ("HttpRouteRulesBackendRefsFiltersExternalAuthProtocol", "Http"),
        (
            "HttpRouteRulesBackendRefsFiltersRequestRedirectPathType",
            "ReplaceFullPath",
        ),
        ("HttpRouteRulesBackendRefsFiltersType", "RequestHeaderModifier"),
        ("HttpRouteRulesBackendRefsFiltersUrlRewritePathType", "ReplaceFullPath"),
        ("HttpRouteRulesFiltersExternalAuthProtocol", "Http"),
        ("HttpRouteRulesFiltersRequestRedirectPathType", "ReplaceFullPath"),
        ("HttpRouteRulesFiltersType", "RequestHeaderModifier"),
        ("HttpRouteRulesFiltersUrlRewritePathType", "ReplaceFullPath"),
    ],
    conditions: &[
        ("GatewayClassConditionType", &["Accepted", "SupportedVersion"]),
        (
            "GatewayClassConditionReason",
            &[
                "Accepted",
                "InvalidParameters",
                "Pending",
                "Unsupported",
                "Waiting",
                "SupportedVersion",
                "UnsupportedVersion",
            ],
        ),
        ("GatewayConditionType", &["Programmed", "Accepted", "Ready"]),
        (
            "GatewayConditionReason",
            &[
                "Programmed",
                "Invalid",
                "NoResources",
                "AddressNotAssigned",
                "AddressNotUsable",
                "Accepted",
                "ListenersNotValid",
                "Pending",
                "UnsupportedAddress",
                "InvalidParameters",
                "Ready",
                "ListenersNotReady",
            ],
        ),
        (
            "ListenerConditionType",
            &["Conflicted", "Accepted", "ResolvedRefs", "Programmed", "Ready"],
        ),
        (
            "ListenerConditionReason",
            &[
                "HostnameConflict",
                "ProtocolConflict",
                "NoConflicts",
                "Accepted",
                "PortUnavailable",
                "UnsupportedProtocol",
                "ResolvedRefs",
                "InvalidCertificateRef",
                "InvalidRouteKinds",
                "RefNotPermitted",
                "Programmed",
                "Invalid",
                "Pending",
                "Ready",
            ],
        ),
    ],
};

// -----------------------------------------------------------------------------
// Entrypoint
// -----------------------------------------------------------------------------

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some("generate") = args.get(1).map(String::as_str) {
        let version = args
            .get(2)
            .context("usage: cargo xtask generate <version>\n  e.g. cargo xtask generate v1.4.1")?;
        generate(version)
    } else {
        eprintln!("usage: cargo xtask generate <version>");
        process::exit(1);
    }
}

// -----------------------------------------------------------------------------
// Generation orchestration
// -----------------------------------------------------------------------------

fn generate(version: &str) -> Result<()> {
    for channel in [&STANDARD, &EXPERIMENTAL] {
        generate_channel(version, channel)?;
    }
    write_apis_module()?;
    cargo_fmt()?;
    Ok(())
}

fn generate_channel(version: &str, channel: &Channel) -> Result<()> {
    let channel_name = channel.name;
    let dir = format!("{APIS_DIR}/{channel_name}");

    // Clean and recreate
    let path = Path::new(&dir);
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)?;

    let mut unused_defaults: BTreeSet<&str> = channel.enum_defaults.iter().map(|(name, _)| *name).collect();

    // Fetch CRDs and run kopium once per API version
    for api in channel.apis {
        let api_name = api.name;
        let root_version = api.root_version;
        let crd = fetch_crd(version, channel_name, api)?;

        if !crd.spec.versions.iter().any(|v| v.name == root_version) {
            bail!("{channel_name} {api_name}: root version {root_version} is not defined by the CRD");
        }

        fs::create_dir_all(format!("{dir}/{api_name}"))?;
        for crd_version in &crd.spec.versions {
            generate_version(version, channel, api, &crd, crd_version, &mut unused_defaults)?;
        }

        fs::write(format!("{dir}/{api_name}.rs"), gen_resource_module(api, &crd.spec)?)?;
    }

    if let Some(name) = unused_defaults.first() {
        bail!("{channel_name}: enum default for {name} matches no generated enum");
    }

    // Generate supporting files
    let channel_module = gen_channel_module(channel)?;
    let constants = gen_constants(channel.conditions)?;

    fs::write(format!("{dir}.rs"), channel_module)?;
    fs::write(format!("{dir}/constants.rs"), constants)?;

    Ok(())
}

/// Runs kopium for one API version of a CRD and writes its module file.
fn generate_version(
    bundle: &str,
    channel: &Channel,
    api: &Api,
    crd: &FetchedCrd,
    crd_version: &CrdVersion,
    unused_defaults: &mut BTreeSet<&'static str>,
) -> Result<()> {
    let spec = &crd.spec;
    let channel_name = channel.name;
    let api_name = api.name;
    let api_version = &crd_version.name;

    eprintln!("generating {channel_name} api {api_name} {api_version}");
    let kopium_rs = kopium(&crd.yaml, api_version)
        .with_context(|| format!("kopium failed for {channel_name} {api_name} {api_version}"))?;

    let mut rs = gen_version_doc(bundle, channel, spec, crd_version);
    rs.push_str(&kopium_rs);
    rs.push_str(&gen_enum_defaults(&kopium_rs, channel.enum_defaults, unused_defaults)?);
    fs::write(format!("{APIS_DIR}/{channel_name}/{api_name}/{api_version}.rs"), rs)?;

    Ok(())
}

fn write_apis_module() -> Result<()> {
    let out = format!(
        "{GENERATED_HEADER}\
         #[cfg(feature = \"experimental\")]\n\
         #[cfg_attr(docsrs, doc(cfg(feature = \"experimental\")))]\n\
         pub mod experimental;\n\
         pub mod standard;\n"
    );

    fs::write(format!("{APIS_DIR}.rs"), out)?;
    Ok(())
}

fn cargo_fmt() -> Result<()> {
    let status = Command::new("cargo").args(["+nightly", "fmt"]).status()?;
    if !status.success() {
        bail!("cargo fmt failed");
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// CRD fetching and kopium conversion
// -----------------------------------------------------------------------------

/// Fetches a URL and returns the response body as a string.
fn fetch_url(url: &str) -> Result<String> {
    ureq::get(url)
        .call()
        .with_context(|| format!("failed to fetch {url}"))?
        .into_body()
        .read_to_string()
        .with_context(|| format!("failed to read response from {url}"))
}

/// A fetched CRD: its YAML for kopium, and the parsed parts of its spec.
struct FetchedCrd {
    yaml: String,
    spec: CrdSpec,
}

fn fetch_crd(version: &str, channel: &str, api: &Api) -> Result<FetchedCrd> {
    let Api { name, crd_prefix, .. } = api;
    let url = format!("{CRD_BASE_URL}/{version}/{CRD_PATH}/{channel}/{crd_prefix}{name}.yaml");

    let yaml = fetch_url(&url)?;
    let crd: Crd = serde_saphyr::from_str(&yaml).with_context(|| format!("failed to parse {url}"))?;
    Ok(FetchedCrd { yaml, spec: crd.spec })
}

/// Runs kopium on a CRD for a single API version.
fn kopium(crd_yaml: &str, api_version: &str) -> Result<String> {
    let mut kopium = Command::new("kopium")
        .args(KOPIUM_ARGS)
        .args(["--api-version", api_version])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to spawn kopium — is it installed?")?;

    // Write CRD YAML to kopium stdin, then drop to close the pipe.
    {
        use std::io::Write;
        let mut stdin = kopium.stdin.take().expect("stdin was piped");
        stdin
            .write_all(crd_yaml.as_bytes())
            .context("failed to write CRD to kopium stdin")?;
    }

    let output = kopium.wait_with_output().context("kopium did not exit cleanly")?;

    if !output.status.success() {
        let status = output.status;
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("kopium exited with {status}: {stderr}");
    }

    Ok(String::from_utf8(output.stdout)?)
}

// -----------------------------------------------------------------------------
// Channel module generation
// -----------------------------------------------------------------------------

fn gen_channel_module(channel: &Channel) -> Result<String> {
    let mut out = String::from(GENERATED_HEADER);
    for Api { name, .. } in channel.apis {
        writeln!(out, "pub mod {name};")?;
    }
    out.push_str("pub mod constants;\n");
    Ok(out)
}

// -----------------------------------------------------------------------------
// Resource and API version module generation
// -----------------------------------------------------------------------------

/// The parts of a CRD the generator needs.
#[derive(Deserialize)]
struct Crd {
    spec: CrdSpec,
}

#[derive(Deserialize)]
struct CrdSpec {
    group: String,
    names: CrdNames,
    versions: Vec<CrdVersion>,
}

#[derive(Deserialize)]
struct CrdNames {
    kind: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CrdVersion {
    name: String,
    served: bool,
    storage: bool,
    #[serde(default)]
    deprecated: bool,
    deprecation_warning: Option<String>,
}

/// Generates the resource module, which declares one submodule per API version
/// and re-exports the root version.
fn gen_resource_module(api: &Api, spec: &CrdSpec) -> Result<String> {
    let root_version = api.root_version;
    let kind = &spec.names.kind;
    let group = &spec.group;

    let mut out = format!(
        "{GENERATED_HEADER}\
         //! `{kind}` (`{group}`).\n\
         //!\n\
         //! Each API version the CRD defines is a submodule. This module re-exports [`{root_version}`].\n\
         \n"
    );
    for CrdVersion { name, .. } in &spec.versions {
        writeln!(out, "pub mod {name};")?;
    }
    writeln!(out, "pub use {root_version}::*;")?;
    Ok(out)
}

/// Generates the module docs for an API version, stating its status in the CRD.
fn gen_version_doc(bundle: &str, channel: &Channel, spec: &CrdSpec, v: &CrdVersion) -> String {
    let yes_no = |b: bool| if b { "yes" } else { "no" };
    let channel_name = channel.name;
    let kind = &spec.names.kind;
    let group = &spec.group;
    let api_version = &v.name;
    let served = yes_no(v.served);
    let storage = yes_no(v.storage);
    let deprecated = match (&v.deprecation_warning, v.deprecated) {
        (Some(warning), true) => {
            let warning = warning.replace('\n', " ");
            format!("yes ({warning})")
        },
        _ => yes_no(v.deprecated).to_string(),
    };

    format!(
        "//! `{kind}` API version `{group}/{api_version}`.\n\
         //!\n\
         //! In the Gateway API {bundle} {channel_name} channel CRD:\n\
         //!\n\
         //! - served: {served}\n\
         //! - storage: {storage}\n\
         //! - deprecated: {deprecated}\n\
         \n"
    )
}

/// Generates `Default` impls for the enums in `kopium_rs` that kopium cannot
/// derive one for, removing each matched entry from `unused`.
fn gen_enum_defaults(
    kopium_rs: &str,
    defaults: &[(&'static str, &str)],
    unused: &mut BTreeSet<&'static str>,
) -> Result<String> {
    let mut out = String::new();
    for (name, default) in defaults {
        if !kopium_rs.contains(&format!("pub enum {name} {{")) {
            continue;
        }
        write!(
            out,
            "\nimpl Default for {name} {{\n    \
                 fn default() -> Self {{\n        \
                     {name}::{default}\n    \
                 }}\n\
             }}\n"
        )?;
        unused.remove(name);
    }
    Ok(out)
}

// -----------------------------------------------------------------------------
// constants.rs generation
// -----------------------------------------------------------------------------

fn gen_constants(conditions: &[(&str, &[&str])]) -> Result<String> {
    let mut out = String::from(GENERATED_HEADER);

    for (name, variants) in conditions {
        // Enum definition
        write!(out, "\n#[derive(Debug, PartialEq, Eq)]\npub enum {name} {{\n")?;
        for v in *variants {
            writeln!(out, "    {v},")?;
        }
        out.push_str("}\n");

        // Display impl
        write!(
            out,
            "\nimpl std::fmt::Display for {name} {{\n    \
                 fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{\n        \
                     write!(f, \"{{:?}}\", self)\n    \
                 }}\n\
             }}\n"
        )?;
    }

    Ok(out)
}
