//! Schema conformance of the crate's types against the vendored upstream CRDs.
//!
//! This is a structural comparison: the crate's CRDs must describe the same wire
//! shape as upstream's (fields, types and formats, required fields, enum
//! values). Validations such as CEL rules and patterns are left to the API
//! server.
//!
//! See [`Attribute`] for the attributes compared, and [`ALLOWED`] for known
//! differences.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::OnceLock,
};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    CustomResourceDefinition, JSONSchemaProps, JSONSchemaPropsOrArray, JSONSchemaPropsOrBool,
};
use kube::{CustomResourceExt, Resource};

/// Declares a module per channel, resource and API version with a
/// `matches_upstream` test, e.g. `standard::httproutes::v1::matches_upstream`,
/// that checks the version's root type, and a `COMPARISONS` list of every
/// version's comparison for the tests across all of them. The test bodies
/// in the table are for the integration tests.
macro_rules! channel {
    ($channel:ident { $($resource:ident: $root:ident { $($version:ident => $body:literal),+ $(,)? })+ }) => {
        mod $channel {
            pub(super) const COMPARISONS: &[fn() -> crate::Comparison] = &[$($(
                || crate::compare::<gateway_api::apis::$channel::$resource::$version::$root>(
                    crate::Channel::named(stringify!($channel)),
                    stringify!($resource),
                    stringify!($version),
                ),
            )+)+];

            $(
                mod $resource {
                    $(
                        mod $version {
                            #[test]
                            fn matches_upstream() {
                                crate::check(crate::compare::<
                                    gateway_api::apis::$channel::$resource::$version::$root,
                                >(
                                    crate::Channel::named(stringify!($channel)),
                                    stringify!($resource),
                                    stringify!($version),
                                ));
                            }
                        }
                    )+
                }
            )+
        }
    };
}

include!("integration/channels.rs");

// The shared state below is in a `OnceLock` rather than a `LazyLock`: if
// initializing panics, the next test retries and reports the same panic,
// instead of a poisoned lock.

/// Every upstream CRD in the channels this build checks, by channel and
/// resource, parsed once for all tests from the fixtures named
/// `<group>_<resource>.yaml`.
fn upstream() -> &'static BTreeMap<(Channel, String), CustomResourceDefinition> {
    static UPSTREAM: OnceLock<BTreeMap<(Channel, String), CustomResourceDefinition>> = OnceLock::new();
    UPSTREAM.get_or_init(|| {
        let mut crds = BTreeMap::new();
        for &channel in Channel::BUILT {
            let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/crds")
                .join(channel.name());
            let entries =
                fs::read_dir(&dir).unwrap_or_else(|e| panic!("failed to read {dir}: {e}", dir = dir.display()));
            for entry in entries {
                let path = entry.expect("failed to read a directory entry").path();
                let resource = path
                    .file_stem()
                    .and_then(|stem| stem.to_str()?.split_once('_'))
                    .unwrap_or_else(|| panic!("{path} isn't named <group>_<resource>.yaml", path = path.display()))
                    .1;
                let yaml = fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("failed to read {path}: {e}", path = path.display()));
                let crd = serde_saphyr::from_str(&yaml)
                    .unwrap_or_else(|e| panic!("failed to parse {path}: {e}", path = path.display()));
                crds.insert((channel, resource.to_owned()), crd);
            }
        }
        crds
    })
}

/// Every API version's comparison in the channels this build checks.
fn all_comparisons() -> &'static [Comparison] {
    static ALL_COMPARISONS: OnceLock<Vec<Comparison>> = OnceLock::new();
    ALL_COMPARISONS.get_or_init(|| {
        let channels: &[&[fn() -> Comparison]] = &[
            standard::COMPARISONS,
            #[cfg(feature = "experimental")]
            experimental::COMPARISONS,
        ];
        channels.iter().copied().flatten().map(|f| f()).collect()
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Channel {
    Standard,
    Experimental,
}

impl Channel {
    const ALL: [Self; 2] = [Self::Standard, Self::Experimental];
    /// The channels this build checks; `experimental` needs its feature.
    const BUILT: &[Self] = &[
        Self::Standard,
        #[cfg(feature = "experimental")]
        Self::Experimental,
    ];

    /// The channel's module and fixture directory name.
    fn name(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Experimental => "experimental",
        }
    }

    fn named(name: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|c| c.name() == name)
            .unwrap_or_else(|| panic!("unknown channel {name}"))
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

// -----------------------------------------------------------------------------
// Known differences
// -----------------------------------------------------------------------------

/// A known difference between a type's schema and the upstream CRD.
struct Allowed {
    /// The channels the difference is in, each of which it must match.
    channels: &'static [Channel],
    /// The resource's kind, as in the CRD, or `*` for every kind.
    kind: &'static str,
    /// A schema path, as in a failure report, with `""` for the root. A
    /// leading `*` matches any prefix.
    path: &'static str,
    /// The attribute that differs, e.g. `Required` or `Enum`.
    attribute: Attribute,
    /// The upstream value.
    upstream: &'static str,
    /// The crate's value.
    ours: &'static str,
    /// Why the difference is allowed.
    reason: &'static str,
}

/// Differences the check accepts.
const ALLOWED: &[Allowed] = &[
    Allowed {
        channels: &Channel::ALL,
        kind: "*",
        path: "*.conditions[].status",
        attribute: Attribute::Enum,
        upstream: r#"["False", "True", "Unknown"]"#,
        ours: "[]",
        reason: "k8s-openapi's `Condition::status` is a plain string",
    },
    Allowed {
        channels: &Channel::ALL,
        kind: "Gateway",
        path: "*.addresses[]/oneOf",
        attribute: Attribute::Exists,
        upstream: "true",
        ours: "false",
        reason: "kopium doesn't generate validation; address type and value are still compared",
    },
    Allowed {
        channels: &Channel::ALL,
        kind: "HTTPRoute",
        path: "*.filters[].requestRedirect.statusCode",
        attribute: Attribute::Enum,
        upstream: "[301, 302, 303, 307, 308]",
        ours: "[]",
        reason: "kopium can't generate an integer enum, so the field is a plain `i64`",
    },
    Allowed {
        channels: &Channel::ALL,
        kind: "HTTPRoute",
        path: "*.filters[].cors.allowMethods[]",
        attribute: Attribute::Enum,
        upstream: r#"["*", "CONNECT", "DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT", "TRACE"]"#,
        ours: "[]",
        reason: "kopium doesn't generate an enum for array items, so the items are plain strings",
    },
    Allowed {
        channels: &Channel::ALL,
        kind: "ReferenceGrant",
        path: "",
        attribute: Attribute::Required,
        upstream: "[]",
        ours: "[spec]",
        reason: "kube's `CustomResource` derive always requires `spec`; upstream makes it optional",
    },
];

impl Allowed {
    fn matches(&self, comparison: &Comparison, diff: &Diff) -> bool {
        let path = match self.path.strip_prefix('*') {
            Some(suffix) => diff.path.ends_with(suffix),
            None => diff.path == self.path,
        };
        self.channels.contains(&comparison.channel)
            && (self.kind == "*" || self.kind == comparison.kind)
            && path
            && self.attribute == diff.attribute
            && self.upstream == diff.upstream
            && self.ours == diff.ours
    }
}

/// Fails on [`ALLOWED`] entries that match no difference in one of their
/// channels. Channels the build leaves out, i.e. `experimental` without its
/// feature, are skipped.
#[test]
fn allowed_entries_match() {
    let stale = ALLOWED
        .iter()
        .flat_map(|a| a.channels.iter().map(move |&channel| (a, channel)))
        .filter(|(_, channel)| Channel::BUILT.contains(channel))
        .filter(|(a, channel)| {
            !all_comparisons()
                .iter()
                .any(|c| c.channel == *channel && c.diffs.iter().any(|d| a.matches(c, d)))
        })
        .map(|(a, channel)| {
            let Allowed {
                kind,
                path,
                attribute,
                reason,
                ..
            } = a;
            format!("{channel} {kind} {path} {attribute:?} ({reason})")
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(stale.is_empty(), "stale ALLOWED entries:\n{stale}");
}

// -----------------------------------------------------------------------------
// Check
// -----------------------------------------------------------------------------

/// One API version's CRD compared with the upstream one.
struct Comparison {
    channel: Channel,
    resource: String,
    kind: String,
    version: String,
    diffs: Vec<Diff>,
}

/// Compares the CRD that `K` derives with the upstream CRD for `resource` in
/// `channel`, at `version`.
fn compare<K>(channel: Channel, resource: &str, version: &str) -> Comparison
where
    K: CustomResourceExt + Resource<DynamicType = ()>,
{
    let kind = K::kind(&()).into_owned();
    let upstream_crd = upstream()
        .get(&(channel, resource.to_owned()))
        .unwrap_or_else(|| panic!("no upstream CRD for {channel} {resource}"));
    assert!(
        upstream_crd.spec.versions.iter().any(|v| v.name == version),
        "upstream CRD for {channel} {resource} doesn't define {version}"
    );
    Comparison {
        channel,
        resource: resource.to_owned(),
        diffs: diff(
            &attributes(upstream_crd, version),
            &attributes(&K::crd(), &K::version(&())),
        ),
        kind,
        version: version.to_owned(),
    }
}

/// Fails on API versions an upstream CRD defines that the tables don't
/// compare.
#[test]
fn every_upstream_version_is_compared() {
    let compared: BTreeSet<_> = all_comparisons()
        .iter()
        .map(|c| (c.channel, c.resource.as_str(), c.version.as_str()))
        .collect();
    let missing = upstream()
        .iter()
        .flat_map(|((channel, resource), crd)| {
            crd.spec
                .versions
                .iter()
                .map(move |v| (*channel, resource.as_str(), v.name.as_str()))
        })
        .filter(|key| !compared.contains(key))
        .map(|(channel, resource, version)| format!("{channel} {resource} {version}"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(missing.is_empty(), "upstream API versions the tables miss:\n{missing}");
}

/// Fails on differences that [`ALLOWED`] doesn't accept.
fn check(comparison: Comparison) {
    let report = comparison
        .diffs
        .iter()
        .filter(|d| !ALLOWED.iter().any(|a| a.matches(&comparison, d)))
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    let Comparison {
        channel, kind, version, ..
    } = comparison;
    assert!(
        report.is_empty(),
        "{channel} {kind} {version} differs from the upstream CRD:\n{report}"
    );
}

// -----------------------------------------------------------------------------
// Attributes
// -----------------------------------------------------------------------------

/// The attributes recorded at one path, and their values as text.
type Node = BTreeMap<Attribute, String>;

/// The attributes of one API version of a CRD, as a [`Node`] per path:
/// `(crd)` for the CRD itself and `""` for the schema's root.
fn attributes(crd: &CustomResourceDefinition, version: &str) -> BTreeMap<String, Node> {
    let spec = &crd.spec;
    let kind = &spec.names.kind;
    let v = spec
        .versions
        .iter()
        .find(|v| v.name == version)
        .unwrap_or_else(|| panic!("{kind} CRD doesn't define {version}"));
    let schema = v
        .schema
        .as_ref()
        .and_then(|s| s.open_api_v3_schema.as_ref())
        .unwrap_or_else(|| panic!("{kind} {version} has no schema"));

    let status_subresource = v.subresources.as_ref().is_some_and(|s| s.status.is_some());
    let mut nodes = BTreeMap::from([(
        "(crd)".into(),
        Node::from([
            (Attribute::Group, spec.group.clone()),
            (Attribute::Version, v.name.clone()),
            (Attribute::Kind, spec.names.kind.clone()),
            (Attribute::Plural, spec.names.plural.clone()),
            (Attribute::Scope, spec.scope.clone()),
            (Attribute::StatusSubresource, status_subresource.to_string()),
        ]),
    )]);
    walk(schema, "", &mut nodes);
    nodes
}

/// Records the node for `schema` at `path`, then for its properties, map
/// values, array items and combinators (`allOf`, `anyOf`, `oneOf`, `not`).
fn walk(schema: &JSONSchemaProps, path: &str, nodes: &mut BTreeMap<String, Node>) {
    let type_ = schema.type_.as_deref().unwrap_or("(none)");
    // The API server reads an integer without a format as an `int64`, which
    // is also what kopium generates for one.
    let format = match (type_, schema.format.as_deref()) {
        ("integer", None) => "int64",
        (_, format) => format.unwrap_or("(none)"),
    };
    let required = schema.required.iter().flatten();
    // Enum values as JSON, so `"301"` and `301` differ. schemars lists `null`
    // for an `Option`, which `required` already covers.
    let enum_values = schema.enum_.iter().flatten().filter(|v| !v.0.is_null());
    // A schema here is walked as `{*}` below.
    let additional_properties = match schema.additional_properties {
        Some(JSONSchemaPropsOrBool::Bool(b)) => b.to_string(),
        _ => "(none)".into(),
    };
    nodes.insert(
        path.into(),
        Node::from([
            (Attribute::Type, type_.into()),
            (Attribute::Format, format.into()),
            (Attribute::Required, list(required.cloned())),
            (Attribute::Enum, list(enum_values.map(|v| v.0.to_string()))),
            (
                Attribute::IntOrString,
                (schema.x_kubernetes_int_or_string == Some(true)).to_string(),
            ),
            (
                Attribute::PreserveUnknownFields,
                (schema.x_kubernetes_preserve_unknown_fields == Some(true)).to_string(),
            ),
            (Attribute::AdditionalProperties, additional_properties),
        ]),
    );

    for (name, property) in schema.properties.iter().flatten() {
        // Upstream's schemas describe the object's own `apiVersion`, `kind`
        // and `metadata`; kube's derived schemas only describe `spec` and
        // `status`. The API server handles all three itself either way.
        if path.is_empty() && matches!(name.as_str(), "apiVersion" | "kind" | "metadata") {
            continue;
        }
        walk(property, &format!("{path}.{name}"), nodes);
    }
    if let Some(JSONSchemaPropsOrBool::Schema(values)) = &schema.additional_properties {
        walk(values, &format!("{path}{{*}}"), nodes);
    }
    match &schema.items {
        Some(JSONSchemaPropsOrArray::Schema(items)) => walk(items, &format!("{path}[]"), nodes),
        Some(JSONSchemaPropsOrArray::Schemas(items)) => {
            for (i, item) in items.iter().enumerate() {
                walk(item, &format!("{path}[{i}]"), nodes);
            }
        },
        None => {},
    }
    // A combinator is a node of its own, so one missing on one side is one
    // difference, not one per branch.
    for (name, branches) in [
        ("allOf", &schema.all_of),
        ("anyOf", &schema.any_of),
        ("oneOf", &schema.one_of),
    ] {
        if let Some(branches) = branches {
            let path = format!("{path}/{name}");
            for (i, branch) in branches.iter().enumerate() {
                walk(branch, &format!("{path}[{i}]"), nodes);
            }
            nodes.insert(path, Node::new());
        }
    }
    if let Some(not) = &schema.not {
        walk(not, &format!("{path}/not"), nodes);
    }
}

/// Renders `items` as a set, sorted and without duplicates, as `[a, b]`.
fn list(items: impl IntoIterator<Item = String>) -> String {
    let mut items: Vec<_> = items.into_iter().collect();
    items.sort();
    items.dedup();
    format!("[{items}]", items = items.join(", "))
}

// -----------------------------------------------------------------------------
// Diff
// -----------------------------------------------------------------------------

/// Something compared at a path, and what a difference is in. `Exists` is
/// never recorded in a [`Node`]: it's the difference when a path is on one
/// side only. `nullable` isn't compared: the derived schemas mark every
/// `Option` field nullable and upstream marks none, but the generated fields
/// skip `None` when serializing, so the types never send `null`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Attribute {
    Group,
    Version,
    Kind,
    Plural,
    Scope,
    StatusSubresource,
    Exists,
    Type,
    Format,
    Required,
    Enum,
    IntOrString,
    PreserveUnknownFields,
    AdditionalProperties,
}

struct Diff {
    /// A schema path, `""` for the root, or `(crd)` for the CRD itself.
    path: String,
    attribute: Attribute,
    upstream: String,
    ours: String,
}

impl std::fmt::Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = if self.path.is_empty() { "(root)" } else { &self.path };
        let Self {
            attribute,
            upstream,
            ours,
            ..
        } = self;
        write!(f, "{path} {attribute:?}: upstream {upstream}, ours {ours}")
    }
}

fn diff(upstream: &BTreeMap<String, Node>, ours: &BTreeMap<String, Node>) -> Vec<Diff> {
    let mut diffs = Vec::new();
    let mut missing = BTreeSet::new();
    let paths: BTreeSet<_> = upstream.keys().chain(ours.keys()).collect();
    for path in paths {
        // A path on one side only is one difference; its descendants, whose
        // ancestors end where a separator starts, aren't more.
        let mut ancestors = path.match_indices(['.', '[', '{', '/']).map(|(i, _)| &path[..i]);
        if ancestors.any(|a| missing.contains(a)) {
            continue;
        }
        let (Some(u), Some(o)) = (upstream.get(path), ours.get(path)) else {
            missing.insert(path.as_str());
            diffs.push(Diff {
                path: path.clone(),
                attribute: Attribute::Exists,
                upstream: upstream.contains_key(path).to_string(),
                ours: ours.contains_key(path).to_string(),
            });
            continue;
        };
        // Nodes at the same path have the same attributes.
        for (&attribute, u) in u {
            if *u != o[&attribute] {
                diffs.push(Diff {
                    path: path.clone(),
                    attribute,
                    upstream: u.clone(),
                    ours: o[&attribute].clone(),
                });
            }
        }
    }
    diffs
}
