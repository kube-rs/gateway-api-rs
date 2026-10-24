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
    thread,
};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

const APIS_DIR: &str = "gateway-api/src/apis";

/// Vendored upstream CRDs, installed by the integration tests.
const CRD_FIXTURES_DIR: &str = "gateway-api/tests/fixtures/crds";

/// Integration test bodies, shared across API versions and channels.
const TEST_SUITE_DIR: &str = "gateway-api/tests/integration/suite";

/// Integration test tables, included by the integration tests' `main.rs`.
const TEST_TABLES_FILE: &str = "gateway-api/tests/integration/channels.rs";

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

/// API versions a channel's CRDs define, per resource.
type ChannelVersions = Vec<(&'static str, Vec<String>)>;

fn generate(version: &str) -> Result<()> {
    let mut channels = Vec::new();
    for channel in [&STANDARD, &EXPERIMENTAL] {
        channels.push((channel, generate_channel(version, channel)?));
    }
    write_apis_module()?;
    fs::write(TEST_TABLES_FILE, gen_test_tables(&channels)?)?;
    rustfmt()?;
    Ok(())
}

fn generate_channel(version: &str, channel: &Channel) -> Result<ChannelVersions> {
    let channel_name = channel.name;
    let dir = format!("{APIS_DIR}/{channel_name}");
    let fixtures_dir = format!("{CRD_FIXTURES_DIR}/{channel_name}");

    // Clean and recreate
    for dir in [&dir, &fixtures_dir] {
        let path = Path::new(dir);
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
        fs::create_dir_all(path)?;
    }

    let mut unused_defaults: BTreeSet<&str> = channel.enum_defaults.iter().map(|(name, _)| *name).collect();
    let mut versions = ChannelVersions::new();

    // Fetch CRDs and run kopium once per API version
    for api in channel.apis {
        let api_name = api.name;
        let root_version = api.root_version;
        let crd = fetch_crd(version, channel_name, api)?;
        fs::write(
            format!("{fixtures_dir}/{}{api_name}.yaml", api.crd_prefix),
            gen_crd_fixture(version, &crd),
        )?;

        let Some(root) = crd.spec.versions.iter().find(|v| v.name == root_version) else {
            bail!("{channel_name} {api_name}: root version {root_version} is not defined by the CRD");
        };
        if !root.served {
            bail!("{channel_name} {api_name}: root version {root_version} is not served by the CRD");
        }

        fs::create_dir_all(format!("{dir}/{api_name}"))?;
        for crd_version in &crd.spec.versions {
            generate_version(version, channel, api, &crd, crd_version, &mut unused_defaults)?;
        }

        fs::write(format!("{dir}/{api_name}.rs"), gen_resource_module(api, &crd.spec))?;
        versions.push((api_name, crd.spec.versions.into_iter().map(|v| v.name).collect()));
    }

    if let Some(name) = unused_defaults.first() {
        bail!("{channel_name}: enum default for {name} matches no generated enum");
    }

    // Generate supporting files
    let channel_module = gen_channel_module(channel);
    let constants = gen_constants(channel.conditions);

    fs::write(format!("{dir}.rs"), channel_module)?;
    fs::write(format!("{dir}/constants.rs"), constants)?;

    Ok(versions)
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
    rs.push_str(&gen_enum_defaults(&kopium_rs, channel.enum_defaults, unused_defaults));
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

/// Formats the generated bindings: rustfmt follows the `mod` declarations from
/// their root module into every generated file.
fn rustfmt() -> Result<()> {
    let status = Command::new("rustfmt")
        .args(["+nightly", &format!("{APIS_DIR}.rs")])
        .status()?;
    if !status.success() {
        bail!("rustfmt failed");
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

/// A fetched CRD: where it came from, its YAML for kopium, and the parsed
/// parts of its spec.
struct FetchedCrd {
    url: String,
    yaml: String,
    spec: CrdSpec,
}

fn fetch_crd(version: &str, channel: &str, api: &Api) -> Result<FetchedCrd> {
    let Api { name, crd_prefix, .. } = api;
    let url = format!("{CRD_BASE_URL}/{version}/{CRD_PATH}/{channel}/{crd_prefix}{name}.yaml");

    let yaml = fetch_url(&url)?;
    let crd: Crd = serde_saphyr::from_str(&yaml).with_context(|| format!("failed to parse {url}"))?;
    Ok(FetchedCrd {
        url,
        yaml,
        spec: crd.spec,
    })
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

    // Write CRD YAML to kopium stdin from another thread while collecting its
    // output, so an early exit is reported with kopium's stderr rather than as
    // a broken pipe. The writer drops stdin when done, closing the pipe.
    let mut stdin = kopium.stdin.take().expect("stdin was piped");
    let (written, output) = thread::scope(|s| {
        let writer = s.spawn(move || {
            use std::io::Write;
            stdin.write_all(crd_yaml.as_bytes())
        });
        let output = kopium.wait_with_output();
        (writer.join().expect("kopium stdin writer panicked"), output)
    });
    let output = output.context("kopium did not exit cleanly")?;

    if !output.status.success() {
        let status = output.status;
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("kopium exited with {status}: {stderr}");
    }
    written.context("failed to write CRD to kopium stdin")?;

    Ok(String::from_utf8(output.stdout)?)
}

// -----------------------------------------------------------------------------
// Channel module generation
// -----------------------------------------------------------------------------

fn gen_channel_module(channel: &Channel) -> String {
    let mut out = String::from(GENERATED_HEADER);
    for Api { name, .. } in channel.apis {
        writeln!(out, "pub mod {name};").unwrap();
    }
    out.push_str("pub mod constants;\n");
    out
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
fn gen_resource_module(api: &Api, spec: &CrdSpec) -> String {
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
        writeln!(out, "pub mod {name};").unwrap();
    }
    writeln!(out, "#[doc(inline)]\npub use {root_version}::*;").unwrap();
    out
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
            let warning = warning.split_whitespace().collect::<Vec<_>>().join(" ");
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
) -> String {
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
        )
        .unwrap();
        unused.remove(name);
    }
    out
}

// -----------------------------------------------------------------------------
// Integration test fixture and table generation
// -----------------------------------------------------------------------------

/// Generates a vendored CRD fixture: the upstream YAML, unchanged, under a
/// comment saying where it came from.
fn gen_crd_fixture(bundle: &str, crd: &FetchedCrd) -> String {
    let FetchedCrd { url, yaml, .. } = crd;
    format!(
        "# WARNING: generated file — do not edit\n\
         # Gateway API {bundle} upstream CRD, vendored unchanged below this header by\n\
         # `cargo xtask generate {bundle}` for the integration tests. Source:\n\
         # {url}\n\
         {yaml}"
    )
}

/// Generates the integration test tables: one `channel!` invocation per
/// channel, mapping every API version each CRD defines to a test body.
fn gen_test_tables(channels: &[(&Channel, ChannelVersions)]) -> Result<String> {
    let mut out = String::from(GENERATED_HEADER);
    for (channel, resources) in channels {
        let channel_name = channel.name;
        out.push('\n');
        if channel_name == EXPERIMENTAL.name {
            out.push_str("#[cfg(feature = \"experimental\")]\n");
        }
        writeln!(out, "channel!({channel_name} {{").unwrap();
        for (resource, versions) in resources {
            let entries = versions
                .iter()
                .map(|version| {
                    let body = test_body(resource, version)?;
                    Ok(format!("{version} => \"{body}\""))
                })
                .collect::<Result<Vec<_>>>()?;
            writeln!(out, "    {resource} {{ {} }}", entries.join(", ")).unwrap();
        }
        out.push_str("});\n");
    }
    Ok(out)
}

/// Finds the test body for an API version: `<resource>_<version>.rs` if it
/// exists, else `<resource>.rs`, shared by the resource's other versions. If
/// neither exists, as for a newly added CRD, writes a `<resource>.rs` stub
/// whose test fails with `todo!`.
fn test_body(resource: &str, version: &str) -> Result<String> {
    let suite = Path::new(TEST_SUITE_DIR);
    let specific = format!("{resource}_{version}.rs");
    if suite.join(&specific).exists() {
        return Ok(specific);
    }

    let shared = format!("{resource}.rs");
    let path = suite.join(&shared);
    if !path.exists() {
        eprintln!("writing integration test stub {}", path.display());
        fs::write(&path, gen_test_stub(resource))?;
    }
    Ok(shared)
}

fn gen_test_stub(resource: &str) -> String {
    format!(
        "// TODO: written by `cargo xtask generate` because `{resource}` had no test\n\
         // body. Build a valid object from `super::api` and check it with\n\
         // `common::crud`, like the other bodies in this directory.\n\
         \n\
         #[allow(unused_imports)] // until the test uses `api`\n\
         use super::api as _;\n\
         \n\
         #[ignore]\n\
         #[tokio::test]\n\
         async fn crud() {{\n    \
             todo!(\"integration test for {resource}\")\n\
         }}\n"
    )
}

// -----------------------------------------------------------------------------
// constants.rs generation
// -----------------------------------------------------------------------------

fn gen_constants(conditions: &[(&str, &[&str])]) -> String {
    let mut out = String::from(GENERATED_HEADER);

    for (name, variants) in conditions {
        // Enum definition
        write!(out, "\n#[derive(Debug, PartialEq, Eq)]\npub enum {name} {{\n").unwrap();
        for v in *variants {
            writeln!(out, "    {v},").unwrap();
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
        )
        .unwrap();
    }

    out
}
