use std::{fmt::Debug, fs, path::Path, time::Duration};

use anyhow::{Context, Result, bail, ensure};
use hyper_util::{client::legacy::Client as HttpClient, rt::TokioExecutor};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use kube::{
    Api, Client, Config, Resource, ResourceExt,
    api::{DeleteParams, ListParams, Patch, PatchParams, PostParams},
    client::ConfigExt,
    config::{KubeConfigOptions, Kubeconfig},
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::OnceCell;
use tower::{BoxError, ServiceBuilder};

/// The upstream CRD channel installed for this run. Standard and experimental
/// CRDs share names, so a cluster holds one channel at a time. The standard
/// types are tested against whichever is installed: the experimental schema
/// is a superset of the standard one.
pub const CHANNEL: &str = if cfg!(feature = "experimental") {
    "experimental"
} else {
    "standard"
};

/// Field manager for server-side apply.
const FIELD_MANAGER: &str = "gateway-api-tests";

// -----------------------------------------------------------------------------
// Shared cluster state
// -----------------------------------------------------------------------------

static CLIENT_INIT: OnceCell<String> = OnceCell::const_new();

/// Return a fresh client bound to the current tokio runtime.
/// The cluster is expected to already exist — `KUBECONFIG` env var must point
/// at a valid kubeconfig file. CRDs are installed once on first call.
pub async fn client() -> Client {
    let kubeconfig = CLIENT_INIT
        .get_or_init(|| async {
            let path = std::env::var("KUBECONFIG").expect("KUBECONFIG env var must be set for integration tests");
            let kubeconfig =
                std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read KUBECONFIG at {path}: {e}"));

            let client = make_client(&kubeconfig).await.expect("failed to create kube client");

            install_crds(&client, CHANNEL)
                .await
                .unwrap_or_else(|e| panic!("failed to install {CHANNEL} CRDs: {e:#}"));

            kubeconfig
        })
        .await;

    make_client(kubeconfig).await.expect("failed to create kube client")
}

// -----------------------------------------------------------------------------
// API version checks
// -----------------------------------------------------------------------------

/// Checks one API version of a resource against the installed upstream CRD.
///
/// If the CRD serves the version, `object` is created, read, listed, updated
/// and deleted through `api`. The update is a server-side apply of the object
/// as read back, with strict field validation, so the API server rejects any
/// field the type writes that the upstream schema doesn't define.
///
/// If the CRD defines the version but doesn't serve it, creating `object` must
/// fail with 404.
///
/// `object` is given a generated name, so checks of the same resource through
/// different versions or channels never collide.
pub async fn crud<K>(api: Api<K>, mut object: K)
where
    K: Resource<DynamicType = ()> + Clone + Debug + PartialEq + Serialize + DeserializeOwned,
{
    let resource = format!(
        "{api_version} {kind}",
        api_version = K::api_version(&()),
        kind = K::kind(&())
    );
    object.meta_mut().generate_name = Some("test-".into());

    if !served::<K>(api.clone().into_client()).await {
        match api.create(&PostParams::default(), &object).await {
            // The URL isn't routed at all, so the body is plain text, not a
            // `Status` that `is_not_found` could recognize.
            Err(kube::Error::Api(status)) if status.code == 404 => return,
            Err(e) => panic!("{resource} is not served, expected 404 on create, got: {e}"),
            Ok(_) => panic!("{resource} is not served, but create succeeded"),
        }
    }

    let created = api
        .create(&PostParams::default(), &object)
        .await
        .unwrap_or_else(|e| panic!("{resource}: create failed: {e}"));
    let name = created.name_any();

    let result = read_update_delete(&api, &name, created).await;
    // Clean up even when a step failed. After a successful delete this is a
    // no-op 404.
    let _ = api.delete(&name, &DeleteParams::default()).await;
    result.unwrap_or_else(|e| panic!("{resource} {name}: {e:#}"));
}

async fn read_update_delete<K>(api: &Api<K>, name: &str, created: K) -> Result<()>
where
    K: Resource<DynamicType = ()> + Clone + Debug + PartialEq + Serialize + DeserializeOwned,
{
    let read = api.get(name).await.context("get failed")?;
    ensure!(read == created, "read back {read:?}, created {created:?}");

    let list = api.list(&ListParams::default()).await.context("list failed")?;
    ensure!(
        list.items.iter().any(|o| o.name_any() == name),
        "list does not contain the object"
    );

    let mut apply = read;
    apply.meta_mut().managed_fields = None;
    apply
        .labels_mut()
        .insert("gateway-api-tests/updated".into(), "true".into());
    let params = PatchParams::apply(FIELD_MANAGER).force().validation_strict();
    let updated = api
        .patch(name, &params, &Patch::Apply(&apply))
        .await
        .context("server-side apply failed")?;
    ensure!(
        updated.labels().get("gateway-api-tests/updated").map(String::as_str) == Some("true"),
        "update did not apply the label"
    );

    api.delete(name, &DeleteParams::default())
        .await
        .context("delete failed")?;
    if api.get_opt(name).await.context("get after delete failed")?.is_some() {
        bail!("object still exists after delete");
    }
    Ok(())
}

/// Whether the installed CRD serves `K`'s API version. Panics if the CRD
/// doesn't define that version at all.
async fn served<K: Resource<DynamicType = ()>>(client: Client) -> bool {
    let crd_name = format!("{plural}.{group}", plural = K::plural(&()), group = K::group(&()));
    let version = K::version(&());
    let crd = Api::<CustomResourceDefinition>::all(client)
        .get(&crd_name)
        .await
        .unwrap_or_else(|e| panic!("failed to get CRD {crd_name}: {e}"));
    crd.spec
        .versions
        .iter()
        .find(|v| v.name == version)
        .unwrap_or_else(|| panic!("{CHANNEL} CRD {crd_name} does not define {version}"))
        .served
}

// -----------------------------------------------------------------------------
// Client construction
// -----------------------------------------------------------------------------

async fn make_client(kubeconfig_yaml: &str) -> Result<Client> {
    let kubeconfig = Kubeconfig::from_yaml(kubeconfig_yaml)?;
    let config = Config::from_custom_kubeconfig(kubeconfig, &KubeConfigOptions::default()).await?;

    let https = config.rustls_https_connector()?;
    let http_client = HttpClient::builder(TokioExecutor::new()).build(https);
    let service = ServiceBuilder::new()
        .layer(config.base_uri_layer())
        .option_layer(config.auth_layer()?)
        .map_err(BoxError::from)
        .service(http_client);

    Ok(Client::new(service, config.default_namespace))
}

// -----------------------------------------------------------------------------
// CRD installation
// -----------------------------------------------------------------------------

/// Installs the vendored upstream CRDs of `channel`: the same YAML the
/// bindings are generated from (see `cargo xtask generate`). Server-side apply
/// makes this idempotent, and replaces the other channel's CRDs on a reused
/// cluster.
async fn install_crds(client: &Client, channel: &str) -> Result<()> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/crds")
        .join(channel);
    let api: Api<CustomResourceDefinition> = Api::all(client.clone());
    let params = PatchParams::apply(FIELD_MANAGER).force();

    let mut names = Vec::new();
    for entry in fs::read_dir(&dir).with_context(|| format!("failed to read {dir}", dir = dir.display()))? {
        let path = entry?.path();
        let yaml = fs::read_to_string(&path)?;
        let crd: CustomResourceDefinition =
            serde_saphyr::from_str(&yaml).with_context(|| format!("failed to parse {path}", path = path.display()))?;
        let name = crd.name_any();
        api.patch(&name, &params, &Patch::Apply(&crd))
            .await
            .with_context(|| format!("failed to apply CRD {name}"))?;
        names.push(name);
    }

    for name in &names {
        wait_established(&api, name).await?;
    }
    Ok(())
}

async fn wait_established(api: &Api<CustomResourceDefinition>, name: &str) -> Result<()> {
    for _ in 0..30 {
        let c = api.get(name).await?;
        let established = c
            .status
            .as_ref()
            .and_then(|s| s.conditions.as_ref())
            .and_then(|conds| conds.iter().find(|c| c.type_ == "Established"))
            .is_some_and(|c| c.status == "True");
        if established {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    bail!("timed out waiting for CRD {name} to become established");
}
