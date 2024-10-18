[![Docs](https://img.shields.io/badge/docs-docs.rs-ff69b4.svg)](https://docs.rs/gateway-api/)
[![crates.io](https://img.shields.io/crates/v/gateway-api.svg)](https://crates.io/crates/gateway-api)
[![License](https://img.shields.io/badge/license-mit-blue.svg)](https://raw.githubusercontent.com/kube-rs/gateway-api-rs/main/LICENSE)

> **Warning**: EXPERIMENTAL. **Not ready for production use**.

> **Note**: This library is considered "unofficial" currently. It's not ready
> for production use, and the [Kubernetes SIG Network] community hasn't
> officially endorsed it. It is a goal of this project to become "official" in
> time as the project matures.

[Kubernetes SIG Network]:https://github.com/kubernetes/community/tree/master/sig-network

# Gateway API (Rust)

> **Note**: Currently supports [Gateway API version v1.2.0][gwv]

[Gateway API] is an official [Kubernetes] API for [Layer 7] network routing.
It is the successor to the [Ingress API] but supports both ingress and service
mesh use cases. See the [Gateway API Introduction] for more details.

This project provides bindings in [Rust] for [Kubernetes] [Gateway API].

[gwv]:https://github.com/kubernetes-sigs/gateway-api/releases/tag/v1.2.0
[Gateway API]:https://gateway-api.sigs.k8s.io/
[Kubernetes]:https://kubernetes.io/
[Layer 7]:https://en.wikipedia.org/wiki/Application_layer
[Ingress API]:https://kubernetes.io/docs/concepts/services-networking/ingress/
[Gateway API Introduction]:https://gateway-api.sigs.k8s.io/#introduction
[Rust]:https://rust-lang.org

## Development

This project uses [Kopium] to automatically generate API bindings from upstream
Gateway API. Make sure you install `kopium` locally in order to run the
generator:

```console
$ cargo install kopium
```

After which you can run the `update.sh` script:

```console
$ ./update.sh
```

Check for errors and/or a non-zero exit code, but upon success you should see
updates automatically generated for code in the `gateway-api/src/api` directory
which you can then commit.

[Kopium]:https://github.com/kube-rs/kopium
