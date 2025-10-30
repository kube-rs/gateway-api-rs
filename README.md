[![Docs](https://img.shields.io/badge/docs-docs.rs-ff69b4.svg)](https://docs.rs/gateway-api/)
[![crates.io](https://img.shields.io/crates/v/gateway-api.svg)](https://crates.io/crates/gateway-api)
[![License](https://img.shields.io/badge/license-mit-blue.svg)](https://raw.githubusercontent.com/kube-rs/gateway-api-rs/main/LICENSE)

> **Warning**: EXPERIMENTAL.

# Gateway API (Rust)

[Rust] bindings for [Kubernetes] [Gateway API].

> **Note**: Currently supports [Gateway API version v1.4.0][gwv]

[gwv]:https://github.com/kubernetes-sigs/gateway-api/releases/tag/v1.4.0
[Rust]:https://rust-lang.org
[Kubernetes]:https://kubernetes.io/
[Gateway API]:https://gateway-api.sigs.k8s.io/

## Usage

This library is intended to be paired with [kube-rs]. Use a [Client] to perform
operations on [Gateway API resources]. This enables you to build [Controllers],
so you can create a [Gateway API Implementation] in Rust. See the
`gateway-api/examples/` directory for example code.

[kube-rs]:https://docs.rs/kube/latest/kube
[Client]:https://docs.rs/kube/latest/kube/struct.Client.html
[Gateway API resources]:https://gateway-api.sigs.k8s.io/api-types/gateway/
[Controller]:https://kube.rs/controllers/intro/
[Gateway API Implemention]:https://gateway-api.sigs.k8s.io/implementations/

## Development

This project uses [Kopium] to automatically generate API bindings from upstream
Gateway API. Make sure you install `kopium` locally in order to run the
generator:

```console
$ cargo install kopium --version 0.22.5
```

After which you can run the `update.sh` script:

```console
$ ./update.sh
```

Check for errors and/or a non-zero exit code, but upon success you should see
updates automatically generated for code in the `gateway-api/src/api` directory
which you can then commit.

[Kopium]:https://github.com/kube-rs/kopium

## Contributions

For questions and general discussion, please use the [discussion board].

Contributions are welcome. Please create an issue describing what changes are
desired prior to creating a PR.

Please check our [project board] to see what work has been accepted and is
in need of an owner. The `next` column contains high priority items.

[project board]:https://github.com/orgs/kube-rs/projects/3
[discussion board]:https://github.com/kube-rs/gateway-api-rs/discussions
