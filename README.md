[![Docs](https://img.shields.io/badge/docs-docs.rs-ff69b4.svg)](https://docs.rs/gateway-api/)
[![crates.io](https://img.shields.io/crates/v/gateway-api.svg)](https://crates.io/crates/gateway-api)
[![License](https://img.shields.io/badge/license-mit-blue.svg)](https://raw.githubusercontent.com/kube-rs/gateway-api-rs/main/LICENSE)

# Gateway API (Rust)

[Kubernetes] [Gateway API] support for [Rust].

> **Note**: Currently supports [Gateway API version v1.5.1][gwv]

[gwv]:https://github.com/kubernetes-sigs/gateway-api/releases/tag/v1.5.1
[Rust]:https://rust-lang.org
[Kubernetes]:https://kubernetes.io/
[Gateway API]:https://gateway-api.sigs.k8s.io/

## Usage

Requires a [kube-rs] [Client] to perform create, read, update and delete (CRUD)
operations on [Gateway API resources].

```rust
use gateway_api::httproutes::HttpRoute;
use kube::{Api, Client, ResourceExt, api::ListParams};

let client = Client::try_default().await?;
let routes: Api<HttpRoute> = Api::default_namespaced(client);
for route in routes.list(&ListParams::default()).await? {
    println!("{}", route.name_any());
}
```

[kube-rs]:https://github.com/kube-rs/kube
[Client]:https://docs.rs/kube/latest/kube/struct.Client.html
[Gateway API resources]:https://gateway-api.sigs.k8s.io/api-types/gateway/

### API versions

Every API version a CRD defines is available as a submodule of the resource
module. The module root re-exports the primary version (usually `v1`):

```rust
use gateway_api::referencegrants::ReferenceGrant; // re-export of v1
use gateway_api::referencegrants::v1::ReferenceGrant as ReferenceGrantV1;
use gateway_api::referencegrants::v1beta1::ReferenceGrant as ReferenceGrantV1beta1;
```

### Experimental APIs

APIs from the [experimental channel] are provided under
`gateway_api::experimental` and require the `experimental` feature.

```toml
[dependencies]
gateway-api = { version = "*", features = ["experimental"] }
```

```rust
use gateway_api::experimental::tcproutes::TcpRoute;
use gateway_api::experimental::httproutes::HttpRoute;
```

> **Warning**: Experimental APIs may change or be removed without notice.

[experimental channel]:https://gateway-api.sigs.k8s.io/docs/concepts/versioning/#release-channels

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md).

## Contributing

Contributions are welcome! For larger changes, please open an issue first.
Check the [project board][board] for unassigned tasks, and use the
[discussions board][forum] for questions.

[board]:https://github.com/orgs/kube-rs/projects/3
[forum]:https://github.com/kube-rs/gateway-api-rs/discussions

## License

Licensed under the [MIT License](LICENSE).
