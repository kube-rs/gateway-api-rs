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
operations on [Gateway API resources]. See the `gateway-api/examples/` directory
for detailed (and specific) usage examples.

[kube-rs]:https://github.com/kube-rs/kube
[Gateway API resources]:https://gateway-api.sigs.k8s.io/api-types/gateway/
[Client]:https://docs.rs/kube/latest/kube/struct.Client.html
[Controller]:https://kube.rs/controllers/intro/

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
