#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

use std::{
    collections::BTreeMap,
    env,
    fmt::Write,
    fs,
    path::Path,
    process::{self, Command, Stdio},
};

use anyhow::{Context, Result, bail};

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
}

impl Api {
    const fn standard(name: &'static str) -> Self {
        Self {
            name,
            crd_prefix: CRD_PREFIX_STANDARD,
        }
    }

    const fn experimental(name: &'static str) -> Self {
        Self {
            name,
            crd_prefix: CRD_PREFIX_EXPERIMENTAL,
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
        Api::standard("backendtlspolicies"),
        Api::standard("gatewayclasses"),
        Api::standard("gateways"),
        Api::standard("grpcroutes"),
        Api::standard("httproutes"),
        Api::standard("listenersets"),
        Api::standard("referencegrants"),
        Api::standard("tlsroutes"),
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
        Api::standard("gatewayclasses"),
        Api::standard("gateways"),
        Api::standard("grpcroutes"),
        Api::standard("httproutes"),
        Api::standard("listenersets"),
        Api::standard("referencegrants"),
        Api::standard("tcproutes"),
        Api::standard("tlsroutes"),
        Api::standard("udproutes"),
        Api::experimental("xbackendtrafficpolicies"),
        Api::experimental("xmeshes"),
    ],
    enum_defaults: &[
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
    write_apis_mod()?;
    cargo_fmt()?;
    Ok(())
}

fn generate_channel(version: &str, channel: &Channel) -> Result<()> {
    let dir = format!("{}/{}", APIS_DIR, channel.name);

    // Clean and recreate
    let path = Path::new(&dir);
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)?;

    // Fetch CRDs and run kopium
    for api in channel.apis {
        eprintln!("generating {} api {}", channel.name, api.name);
        let rs = fetch_and_convert(version, channel.name, api)?;
        fs::write(format!("{dir}/{}.rs", api.name), rs)?;
    }

    // Generate supporting files
    let mod_rs = gen_mod_rs(channel);
    let enum_defaults = gen_enum_defaults(channel.enum_defaults);
    let constants = gen_constants(channel.conditions);

    fs::write(format!("{dir}/mod.rs"), mod_rs)?;
    fs::write(format!("{dir}/enum_defaults.rs"), enum_defaults)?;
    fs::write(format!("{dir}/constants.rs"), constants)?;

    Ok(())
}

fn write_apis_mod() -> Result<()> {
    fs::write(
        format!("{APIS_DIR}/mod.rs"),
        "pub mod experimental;\npub mod standard;\n",
    )?;
    Ok(())
}

fn cargo_fmt() -> Result<()> {
    let status = Command::new("cargo").arg("fmt").status()?;
    if !status.success() {
        bail!("cargo fmt failed");
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// CRD fetching and kopium conversion
// -----------------------------------------------------------------------------

fn fetch_and_convert(version: &str, channel: &str, api: &Api) -> Result<String> {
    let url = format!(
        "{CRD_BASE_URL}/{version}/{CRD_PATH}/{channel}/{}{}.yaml",
        api.crd_prefix, api.name,
    );

    let curl = Command::new("curl")
        .args(["-sSL", &url])
        .stdout(Stdio::piped())
        .spawn()
        .context("failed to spawn curl — is it installed?")?;

    let kopium = Command::new("kopium")
        .args(KOPIUM_ARGS)
        .stdin(curl.stdout.unwrap())
        .output()
        .context("failed to run kopium — is it installed?")?;

    if !kopium.status.success() {
        bail!(
            "kopium failed for {channel}/{}: {}",
            api.name,
            String::from_utf8_lossy(&kopium.stderr)
        );
    }

    Ok(String::from_utf8(kopium.stdout)?)
}

// -----------------------------------------------------------------------------
// mod.rs generation
// -----------------------------------------------------------------------------

fn gen_mod_rs(channel: &Channel) -> String {
    let mut out = String::from(GENERATED_HEADER);
    for api in channel.apis {
        writeln!(out, "pub mod {};", api.name).unwrap();
    }
    writeln!(out, "pub mod constants;").unwrap();
    writeln!(out, "mod enum_defaults;").unwrap();
    out
}

// -----------------------------------------------------------------------------
// enum_defaults.rs generation
// -----------------------------------------------------------------------------

fn gen_enum_defaults(defaults: &[(&str, &str)]) -> String {
    let mut out = String::from(GENERATED_HEADER);

    // Group enums by source module for use statements
    let by_module = group_enums_by_module(defaults.iter().map(|(name, _)| *name));
    for (module, names) in &by_module {
        write!(out, "\nuse super::{module}::{{").unwrap();
        for (i, name) in names.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(name);
        }
        out.push_str("};\n");
    }

    // Default impls
    for (name, default) in defaults {
        write!(
            out,
            "\nimpl Default for {name} {{\n    \
                 fn default() -> Self {{\n        \
                     {name}::{default}\n    \
                 }}\n\
             }}\n"
        )
        .unwrap();
    }

    out
}

fn group_enums_by_module<'a>(names: impl Iterator<Item = &'a str>) -> BTreeMap<&'static str, Vec<&'a str>> {
    let mut map: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for name in names {
        let module = enum_module(name);
        map.entry(module).or_default().push(name);
    }
    map
}

fn enum_module(name: &str) -> &'static str {
    if name.starts_with("HttpRoute") {
        "httproutes"
    } else if name.starts_with("GrpcRoute") {
        "grpcroutes"
    } else if name.starts_with("BackendTlsPolicy") {
        "backendtlspolicies"
    } else if name.starts_with("XBackendTrafficPolicy") {
        "xbackendtrafficpolicies"
    } else if name.starts_with("ListenerSet") {
        "listenersets"
    } else {
        panic!("unknown enum prefix: {name}")
    }
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
