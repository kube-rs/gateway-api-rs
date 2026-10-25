//! Schema conformance of the crate's types against the vendored upstream CRDs.
//!
//! This is a structural comparison: the crate's CRDs must describe the same wire
//! shape as upstream's (fields, types and formats, required fields, enum
//! values). Validations such as CEL rules and patterns are left to the API
//! server.
//!
//! See [`Crd`] and [`Schema`] for the attributes compared, and [`ALLOWED`] for
//! known differences.

#[path = "support/lib.rs"]
pub mod support;

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::OnceLock,
};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    CustomResourceDefinition, JSONSchemaProps, JSONSchemaPropsOrArray, JSONSchemaPropsOrBool,
};
use kube::{CustomResourceExt, Resource};
use support::Channel;

/// An [`apis!`] callback that declares a module per channel, resource and API
/// version with a `matches_upstream` test, e.g.
/// `standard::httproutes::v1::matches_upstream`, that checks the version's
/// root type, and a `COMPARISONS` list of every version's comparison for the
/// tests across all of them.
macro_rules! schema_tests {
    ($channel:ident { $($resource:ident: $root:ident [$($version:ident),+])+ }) => {
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

apis!(schema_tests);

// The shared state below is in a `OnceLock` rather than a `LazyLock`: if
// initializing panics, the next test retries and reports the same panic,
// instead of a poisoned lock.

/// Every upstream CRD in the channels this build checks, by channel and
/// resource, parsed once for all tests.
fn upstream() -> &'static BTreeMap<(Channel, String), CustomResourceDefinition> {
    static UPSTREAM: OnceLock<BTreeMap<(Channel, String), CustomResourceDefinition>> = OnceLock::new();
    UPSTREAM.get_or_init(|| {
        Channel::BUILT
            .iter()
            .flat_map(|&channel| {
                support::fixtures::upstream_crds(channel)
                    .into_iter()
                    .map(move |(resource, crd)| ((channel, resource), crd))
            })
            .collect()
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

// -----------------------------------------------------------------------------
// Known differences
// -----------------------------------------------------------------------------

/// A known difference in a [`Schema`] between a type's CRD and the upstream
/// one. Differences in the [`Crd`]'s own attributes are never allowed.
struct Allowed {
    /// The channels the difference is in, each of which it must match.
    channels: &'static [Channel],
    /// The resource's kind, as in the CRD, or `*` for every kind.
    kind: &'static str,
    /// A schema path, as in a failure report, with `""` for the root. A
    /// leading `*` matches any prefix.
    path: &'static str,
    /// The attribute that differs, e.g. `Required` or `Enum`.
    attribute: SchemaAttribute,
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
        attribute: SchemaAttribute::Enum,
        upstream: r#"["False", "True", "Unknown"]"#,
        ours: "[]",
        reason: "k8s-openapi's `Condition::status` is a plain string",
    },
    Allowed {
        channels: &Channel::ALL,
        kind: "Gateway",
        path: "*.addresses[]/oneOf",
        attribute: SchemaAttribute::Exists,
        upstream: "true",
        ours: "false",
        reason: "kopium doesn't generate validation; address type and value are still compared",
    },
    Allowed {
        channels: &Channel::ALL,
        kind: "HTTPRoute",
        path: "*.filters[].requestRedirect.statusCode",
        attribute: SchemaAttribute::Enum,
        upstream: "[301, 302, 303, 307, 308]",
        ours: "[]",
        reason: "kopium can't generate an integer enum, so the field is a plain `i64`",
    },
    Allowed {
        channels: &Channel::ALL,
        kind: "HTTPRoute",
        path: "*.filters[].cors.allowMethods[]",
        attribute: SchemaAttribute::Enum,
        upstream: r#"["*", "CONNECT", "DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT", "TRACE"]"#,
        ours: "[]",
        reason: "kopium doesn't generate an enum for array items, so the items are plain strings",
    },
    Allowed {
        channels: &Channel::ALL,
        kind: "ReferenceGrant",
        path: "",
        attribute: SchemaAttribute::Required,
        upstream: "[]",
        ours: "[spec]",
        reason: "kube's `CustomResource` derive always requires `spec`; upstream makes it optional",
    },
];

impl Allowed {
    fn matches(&self, comparison: &Comparison, diff: &Diff) -> bool {
        match diff {
            // Differences in the CRD itself are never allowed.
            Diff::Crd { .. } => false,
            Diff::Schema {
                path,
                attribute,
                upstream,
                ours,
            } => {
                let path = match self.path.strip_prefix('*') {
                    Some(suffix) => path.ends_with(suffix),
                    None => path == self.path,
                };
                self.channels.contains(&comparison.channel)
                    && (self.kind == "*" || self.kind == comparison.kind)
                    && path
                    && self.attribute == *attribute
                    && self.upstream == upstream
                    && self.ours == ours
            },
        }
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
        diffs: diff(&Crd::new(upstream_crd, version), &Crd::new(&K::crd(), &K::version(&()))),
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

/// One API version of a CRD, as compared: the CRD's own attributes, and its
/// schema's nodes.
struct Crd {
    group: String,
    version: String,
    kind: String,
    plural: String,
    scope: String,
    status_subresource: bool,
    /// The schema's nodes by path, `""` for the root.
    schemas: BTreeMap<String, Node>,
}

impl Crd {
    /// `version` of `crd`.
    fn new(crd: &CustomResourceDefinition, version: &str) -> Self {
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

        let mut schemas = BTreeMap::new();
        walk(schema, "", &mut schemas);
        Self {
            group: spec.group.clone(),
            version: v.name.clone(),
            kind: kind.clone(),
            plural: spec.names.plural.clone(),
            scope: spec.scope.clone(),
            status_subresource: v.subresources.as_ref().is_some_and(|s| s.status.is_some()),
            schemas,
        }
    }

    /// The CRD's own attributes, as text, without its schemas.
    fn attributes(&self) -> [(CrdAttribute, String); 6] {
        let Self {
            group,
            version,
            kind,
            plural,
            scope,
            status_subresource,
            schemas: _,
        } = self;
        [
            (CrdAttribute::Group, group.clone()),
            (CrdAttribute::Version, version.clone()),
            (CrdAttribute::Kind, kind.clone()),
            (CrdAttribute::Plural, plural.clone()),
            (CrdAttribute::Scope, scope.clone()),
            (CrdAttribute::StatusSubresource, status_subresource.to_string()),
        ]
    }
}

/// What's at one schema path.
enum Node {
    Schema(Schema),
    /// An `allOf`, `anyOf` or `oneOf`, whose branches are nodes of their own.
    /// Only whether it exists is compared, so one missing on one side is one
    /// difference, not one per branch.
    Combinator,
}

/// The characters that start a step in a schema path.
const SEPARATORS: [char; 4] = ['.', '[', '{', '/'];

/// Records the node for `schema` at `path`, then for its properties, map
/// values, array items and combinators (`allOf`, `anyOf`, `oneOf`, `not`).
fn walk(schema: &JSONSchemaProps, path: &str, schemas: &mut BTreeMap<String, Node>) {
    schemas.insert(path.into(), Node::Schema(Schema::from(schema)));

    for (name, property) in schema.properties.iter().flatten() {
        assert!(
            !name.contains(SEPARATORS),
            "{path}.{name}: property name contains a path separator, one of {SEPARATORS:?}"
        );
        // Upstream's schemas describe the object's own `apiVersion`, `kind`
        // and `metadata`; kube's derived schemas only describe `spec` and
        // `status`. The API server handles all three itself either way.
        if path.is_empty() && matches!(name.as_str(), "apiVersion" | "kind" | "metadata") {
            continue;
        }
        walk(property, &format!("{path}.{name}"), schemas);
    }
    if let Some(JSONSchemaPropsOrBool::Schema(values)) = &schema.additional_properties {
        walk(values, &format!("{path}{{*}}"), schemas);
    }
    match &schema.items {
        Some(JSONSchemaPropsOrArray::Schema(items)) => walk(items, &format!("{path}[]"), schemas),
        Some(JSONSchemaPropsOrArray::Schemas(items)) => {
            for (i, item) in items.iter().enumerate() {
                walk(item, &format!("{path}[{i}]"), schemas);
            }
        },
        None => {},
    }
    for (name, branches) in [
        ("allOf", &schema.all_of),
        ("anyOf", &schema.any_of),
        ("oneOf", &schema.one_of),
    ] {
        if let Some(branches) = branches {
            let path = format!("{path}/{name}");
            for (i, branch) in branches.iter().enumerate() {
                walk(branch, &format!("{path}[{i}]"), schemas);
            }
            schemas.insert(path, Node::Combinator);
        }
    }
    if let Some(not) = &schema.not {
        walk(not, &format!("{path}/not"), schemas);
    }
}

/// A schema itself, without its subschemas. `nullable` isn't compared: the
/// derived schemas mark every `Option` field nullable and upstream marks none,
/// but the generated fields skip `None` when serializing, so the types never
/// send `null`.
struct Schema {
    type_: Option<String>,
    format: Option<String>,
    required: BTreeSet<String>,
    /// Enum values as JSON, so `"301"` and `301` differ.
    enum_values: BTreeSet<String>,
    int_or_string: bool,
    preserve_unknown_fields: bool,
    /// Whether other properties are allowed, if set to a bool. A schema here
    /// is walked as `{*}` instead.
    additional_properties: Option<bool>,
}

impl From<&JSONSchemaProps> for Schema {
    fn from(schema: &JSONSchemaProps) -> Self {
        // The API server reads an integer without a format as an `int64`,
        // which is also what kopium generates for one.
        let format = match (schema.type_.as_deref(), &schema.format) {
            (Some("integer"), None) => Some("int64".into()),
            (_, format) => format.clone(),
        };
        let additional_properties = match schema.additional_properties {
            Some(JSONSchemaPropsOrBool::Bool(b)) => Some(b),
            _ => None,
        };
        Self {
            type_: schema.type_.clone(),
            format,
            required: schema.required.iter().flatten().cloned().collect(),
            // schemars lists `null` for an `Option`, which `required` already
            // covers.
            enum_values: schema
                .enum_
                .iter()
                .flatten()
                .filter(|v| !v.0.is_null())
                .map(|v| v.0.to_string())
                .collect(),
            int_or_string: schema.x_kubernetes_int_or_string == Some(true),
            preserve_unknown_fields: schema.x_kubernetes_preserve_unknown_fields == Some(true),
            additional_properties,
        }
    }
}

impl Schema {
    /// The schema's attributes, as text.
    fn attributes(&self) -> [(SchemaAttribute, String); 7] {
        let Self {
            type_,
            format,
            required,
            enum_values,
            int_or_string,
            preserve_unknown_fields,
            additional_properties,
        } = self;
        let none = || "(none)".to_owned();
        [
            (SchemaAttribute::Type, type_.clone().unwrap_or_else(none)),
            (SchemaAttribute::Format, format.clone().unwrap_or_else(none)),
            (SchemaAttribute::Required, list(required)),
            (SchemaAttribute::Enum, list(enum_values)),
            (SchemaAttribute::IntOrString, int_or_string.to_string()),
            (
                SchemaAttribute::PreserveUnknownFields,
                preserve_unknown_fields.to_string(),
            ),
            (
                SchemaAttribute::AdditionalProperties,
                additional_properties.map_or_else(none, |b| b.to_string()),
            ),
        ]
    }
}

/// Renders `items` as `[a, b]`.
fn list(items: &BTreeSet<String>) -> String {
    let items: Vec<_> = items.iter().map(String::as_str).collect();
    format!("[{items}]", items = items.join(", "))
}

// -----------------------------------------------------------------------------
// Diff
// -----------------------------------------------------------------------------

/// An attribute of the CRD itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrdAttribute {
    Group,
    Version,
    Kind,
    Plural,
    Scope,
    StatusSubresource,
}

/// An attribute of a schema. `Exists` is never recorded in a [`Schema`]: it's
/// the difference when a path is on one side only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SchemaAttribute {
    Exists,
    Type,
    Format,
    Required,
    Enum,
    IntOrString,
    PreserveUnknownFields,
    AdditionalProperties,
}

enum Diff {
    /// The CRD itself differs.
    Crd {
        attribute: CrdAttribute,
        upstream: String,
        ours: String,
    },
    /// The schema differs at `path`, `""` for the root.
    Schema {
        path: String,
        attribute: SchemaAttribute,
        upstream: String,
        ours: String,
    },
}

impl std::fmt::Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Crd {
                attribute,
                upstream,
                ours,
            } => write!(f, "CRD {attribute:?}: upstream {upstream}, ours {ours}"),
            Self::Schema {
                path,
                attribute,
                upstream,
                ours,
            } => {
                let path = if path.is_empty() { "(root)" } else { path };
                write!(f, "{path} {attribute:?}: upstream {upstream}, ours {ours}")
            },
        }
    }
}

/// The differences between `upstream` and `ours`: first the CRDs' own
/// attributes, then each schema path.
fn diff(upstream: &Crd, ours: &Crd) -> Vec<Diff> {
    let mut diffs: Vec<_> = changed(upstream.attributes(), ours.attributes())
        .map(|(attribute, upstream, ours)| Diff::Crd {
            attribute,
            upstream,
            ours,
        })
        .collect();

    let mut missing = BTreeSet::new();
    let paths: BTreeSet<_> = upstream.schemas.keys().chain(ours.schemas.keys()).collect();
    for path in paths {
        // A path on one side only is one difference; its descendants, whose
        // ancestors end where a separator starts, aren't more.
        let mut ancestors = path.match_indices(SEPARATORS).map(|(i, _)| &path[..i]);
        if ancestors.any(|a| missing.contains(a)) {
            continue;
        }
        let (u, o) = match (upstream.schemas.get(path), ours.schemas.get(path)) {
            (Some(Node::Schema(u)), Some(Node::Schema(o))) => (u, o),
            (Some(Node::Combinator), Some(Node::Combinator)) => continue,
            // Only a combinator's path ends in `/allOf`, `/anyOf` or `/oneOf`:
            // `walk` rejects property names that would.
            (Some(_), Some(_)) => unreachable!("{path} is a schema on one side and a combinator on the other"),
            (u, o) => {
                missing.insert(path.as_str());
                diffs.push(Diff::Schema {
                    path: path.clone(),
                    attribute: SchemaAttribute::Exists,
                    upstream: u.is_some().to_string(),
                    ours: o.is_some().to_string(),
                });
                continue;
            },
        };
        diffs.extend(
            changed(u.attributes(), o.attributes()).map(|(attribute, upstream, ours)| Diff::Schema {
                path: path.clone(),
                attribute,
                upstream,
                ours,
            }),
        );
    }
    diffs
}

/// The attributes whose values differ, as `(attribute, upstream, ours)`. Both
/// sides must list the same attributes in the same order.
fn changed<A: PartialEq + std::fmt::Debug, const N: usize>(
    upstream: [(A, String); N],
    ours: [(A, String); N],
) -> impl Iterator<Item = (A, String, String)> {
    upstream.into_iter().zip(ours).filter_map(|((u_a, u), (o_a, o))| {
        assert_eq!(u_a, o_a, "attributes listed in a different order");
        (u != o).then_some((u_a, u, o))
    })
}
