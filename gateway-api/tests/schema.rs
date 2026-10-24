//! Schema conformance of the crate's types against the vendored upstream CRDs.
//!
//! For every channel, resource and API version in `integration/channels.rs`,
//! compares the CRD the type derives (`K::crd()`) with the upstream one. The
//! derived schema comes from the same serde attributes that (de)serialize the
//! type, so it describes what the type puts on the wire. No cluster needed.
//!
//! Only the shape is compared: property names, types, required properties,
//! enum values, maps and arrays. Descriptions, CEL rules, patterns, bounds,
//! formats, defaults and list merge strategies are left to the API server.
//! Known differences are listed in [`ALLOWED`], each with its reason.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    CustomResourceDefinition, JSONSchemaProps, JSONSchemaPropsOrArray, JSONSchemaPropsOrBool,
};
use kube::{CustomResourceExt, Resource};

/// Declares a module per channel and resource with a `#[test]` per API
/// version, e.g. `standard::httproutes::v1beta1`, that checks the version's
/// root type. The test bodies in the table are for the integration tests.
macro_rules! channel {
    ($channel:ident { $($resource:ident: $root:ident { $($version:ident => $body:literal),+ $(,)? })+ }) => {
        mod $channel {
            $(
                mod $resource {
                    $(
                        #[test]
                        fn $version() {
                            crate::check::<gateway_api::apis::$channel::$resource::$version::$root>(
                                stringify!($channel),
                            );
                        }
                    )+
                }
            )+
        }
    };
}

include!("integration/channels.rs");

// -----------------------------------------------------------------------------
// Known differences
// -----------------------------------------------------------------------------

/// A known difference between a type's schema and the upstream CRD.
struct Allowed {
    /// The resource's kind, or `*` for every kind.
    kind: &'static str,
    /// A schema path, as in a failure report, with `""` for the root. A
    /// leading `*` matches any prefix.
    path: &'static str,
    fact: Fact,
    reason: &'static str,
}

/// Differences the check accepts. An entry for one kind that matches nothing
/// in a version of that kind fails that version's test, so stale entries are
/// noticed.
const ALLOWED: &[Allowed] = &[
    Allowed {
        kind: "*",
        path: "*.conditions[].status",
        fact: Fact::Enum,
        reason: "k8s-openapi's `Condition::status` is a plain string",
    },
    Allowed {
        kind: "HTTPRoute",
        path: "*.filters[].requestRedirect.statusCode",
        fact: Fact::Enum,
        reason: "kopium can't generate an integer enum, so the field is a plain `i64`",
    },
    Allowed {
        kind: "HTTPRoute",
        path: "*.filters[].cors.allowMethods[]",
        fact: Fact::Enum,
        reason: "kopium doesn't generate an enum for array items, so the items are plain strings",
    },
    Allowed {
        kind: "ReferenceGrant",
        path: "",
        fact: Fact::Required,
        reason: "kube's `CustomResource` derive always requires `spec`; upstream makes it optional",
    },
];

impl Allowed {
    fn matches(&self, kind: &str, diff: &Diff) -> bool {
        let path = match self.path.strip_prefix('*') {
            Some(suffix) => diff.path.ends_with(suffix),
            None => diff.path == self.path,
        };
        (self.kind == "*" || self.kind == kind) && path && self.fact == diff.fact
    }
}

// -----------------------------------------------------------------------------
// Check
// -----------------------------------------------------------------------------

/// Checks the CRD that `K` derives against the upstream CRD in `channel`.
fn check<K>(channel: &str)
where
    K: CustomResourceExt + Resource<DynamicType = ()>,
{
    let kind = K::kind(&());
    let version = K::version(&());
    let ours = K::crd();
    let upstream = load_upstream(channel, &ours);

    let mut diffs = diff(&Shape::of(&upstream, &version), &Shape::of(&ours, &version));
    let mut report = Vec::new();
    for allowed in ALLOWED.iter().filter(|a| a.kind == kind) {
        if !diffs.iter().any(|d| allowed.matches(&kind, d)) {
            report.push(format!(
                "stale ALLOWED entry: {} {:?} ({})",
                allowed.path, allowed.fact, allowed.reason
            ));
        }
    }
    diffs.retain(|d| !ALLOWED.iter().any(|a| a.matches(&kind, d)));
    report.extend(diffs.iter().map(ToString::to_string));

    assert!(
        report.is_empty(),
        "{channel} {kind} {version} differs from the upstream CRD:\n{}",
        report.join("\n")
    );
}

fn load_upstream(channel: &str, ours: &CustomResourceDefinition) -> CustomResourceDefinition {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/crds")
        .join(channel)
        .join(format!("{}_{}.yaml", ours.spec.group, ours.spec.names.plural));
    let yaml = fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    serde_saphyr::from_str(&yaml).unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()))
}

// -----------------------------------------------------------------------------
// Shapes
// -----------------------------------------------------------------------------

/// What the check compares for one API version of a CRD.
struct Shape {
    group: String,
    kind: String,
    plural: String,
    scope: String,
    status_subresource: bool,
    /// Every schema node, by path, with `""` for the root.
    nodes: BTreeMap<String, Node>,
}

/// What the check compares at one schema path.
#[derive(Default)]
struct Node {
    type_: String,
    /// Required properties, for an object.
    required: BTreeSet<String>,
    /// Enum values as JSON, so `"301"` and `301` differ.
    enum_values: BTreeSet<String>,
    int_or_string: bool,
    preserve_unknown_fields: bool,
}

impl Shape {
    fn of(crd: &CustomResourceDefinition, version: &str) -> Self {
        let spec = &crd.spec;
        let name = &spec.names.kind;
        let v = spec
            .versions
            .iter()
            .find(|v| v.name == version)
            .unwrap_or_else(|| panic!("{name} CRD doesn't define {version}"));
        let schema = v
            .schema
            .as_ref()
            .and_then(|s| s.open_api_v3_schema.as_ref())
            .unwrap_or_else(|| panic!("{name} {version} has no schema"));

        let mut nodes = BTreeMap::new();
        walk(schema, "", &mut nodes);
        Self {
            group: spec.group.clone(),
            kind: spec.names.kind.clone(),
            plural: spec.names.plural.clone(),
            scope: spec.scope.clone(),
            status_subresource: v.subresources.as_ref().is_some_and(|s| s.status.is_some()),
            nodes,
        }
    }
}

/// Records the node for `schema` at `path`, then for its properties, map
/// values and array items.
fn walk(schema: &JSONSchemaProps, path: &str, nodes: &mut BTreeMap<String, Node>) {
    let mut node = Node {
        type_: schema.type_.clone().unwrap_or_default(),
        int_or_string: schema.x_kubernetes_int_or_string == Some(true),
        preserve_unknown_fields: schema.x_kubernetes_preserve_unknown_fields == Some(true),
        ..Node::default()
    };
    // schemars lists `null` for an `Option`, which `required` already covers.
    for value in schema.enum_.iter().flatten().filter(|v| !v.0.is_null()) {
        node.enum_values.insert(value.0.to_string());
    }
    if node.type_ == "object" {
        node.required = schema.required.iter().flatten().cloned().collect();
    }
    nodes.insert(path.into(), node);

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
}

// -----------------------------------------------------------------------------
// Diff
// -----------------------------------------------------------------------------

/// What a difference is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fact {
    Group,
    Kind,
    Plural,
    Scope,
    StatusSubresource,
    Exists,
    Type,
    Required,
    Enum,
    IntOrString,
    PreserveUnknownFields,
}

struct Diff {
    /// A schema path, `""` for the root, or `(crd)` for the CRD itself.
    path: String,
    fact: Fact,
    upstream: String,
    ours: String,
}

impl std::fmt::Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = if self.path.is_empty() { "(root)" } else { &self.path };
        let Self {
            fact, upstream, ours, ..
        } = self;
        write!(f, "{path} {fact:?}: upstream {upstream}, ours {ours}")
    }
}

/// How a compared value reads in a failure report.
trait Show {
    fn show(&self) -> String;
}

impl Show for String {
    fn show(&self) -> String {
        if self.is_empty() { "(none)".into() } else { self.clone() }
    }
}

impl Show for bool {
    fn show(&self) -> String {
        self.to_string()
    }
}

impl Show for BTreeSet<String> {
    fn show(&self) -> String {
        format!("[{}]", self.iter().map(String::as_str).collect::<Vec<_>>().join(", "))
    }
}

fn diff(upstream: &Shape, ours: &Shape) -> Vec<Diff> {
    let mut diffs = Vec::new();
    let mut compare = |path: &str, fact: Fact, upstream: &dyn Show, ours: &dyn Show| {
        if upstream.show() != ours.show() {
            diffs.push(Diff {
                path: path.into(),
                fact,
                upstream: upstream.show(),
                ours: ours.show(),
            });
        }
    };

    compare("(crd)", Fact::Group, &upstream.group, &ours.group);
    compare("(crd)", Fact::Kind, &upstream.kind, &ours.kind);
    compare("(crd)", Fact::Plural, &upstream.plural, &ours.plural);
    compare("(crd)", Fact::Scope, &upstream.scope, &ours.scope);
    compare(
        "(crd)",
        Fact::StatusSubresource,
        &upstream.status_subresource,
        &ours.status_subresource,
    );

    // A path on one side only is one difference; its descendants aren't more.
    let mut missing: Vec<&str> = Vec::new();
    let paths: BTreeSet<_> = upstream.nodes.keys().chain(ours.nodes.keys()).collect();
    for path in paths {
        let descendant = |of: &&str| {
            path.strip_prefix(*of)
                .is_some_and(|rest| rest.starts_with(['.', '[', '{']))
        };
        if missing.iter().any(descendant) {
            continue;
        }
        let (Some(u), Some(o)) = (upstream.nodes.get(path), ours.nodes.get(path)) else {
            compare(
                path,
                Fact::Exists,
                &upstream.nodes.contains_key(path),
                &ours.nodes.contains_key(path),
            );
            missing.push(path);
            continue;
        };
        compare(path, Fact::Type, &u.type_, &o.type_);
        compare(path, Fact::Required, &u.required, &o.required);
        compare(path, Fact::Enum, &u.enum_values, &o.enum_values);
        compare(path, Fact::IntOrString, &u.int_or_string, &o.int_or_string);
        compare(
            path,
            Fact::PreserveUnknownFields,
            &u.preserve_unknown_fields,
            &o.preserve_unknown_fields,
        );
    }
    diffs
}
