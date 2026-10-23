# Changelog

## Unreleased

### Features

- **Bindings for every CRD API version** ([#246]). Each API version a CRD
  defines is now a submodule of its resource module, e.g.
  `gateway_api::httproutes::v1beta1::HttpRoute`. The existing paths are
  unchanged: the module root re-exports the root version. New in v1.5.1:
  - `backendtlspolicies::v1alpha3`
  - `gatewayclasses::v1beta1`
  - `gateways::v1beta1`
  - `httproutes::v1beta1`
  - `referencegrants::v1beta1` (the storage version)
  - `tlsroutes::v1alpha2`, `tlsroutes::v1alpha3`
- **Experimental BackendTLSPolicy** ([#246]).
  `gateway_api::experimental::backendtlspolicies` was missing.

[#246]: https://github.com/kube-rs/gateway-api-rs/issues/246

## 0.22.0 (since v0.21.0)

### Breaking changes

- **Root types renamed to Rust casing** ([#226]). Acronyms in the generated root
  type names now follow the [Rust API naming guidelines]. The Kubernetes `kind`
  values are unchanged.
  - `BackendTLSPolicy` → `BackendTlsPolicy`
  - `GRPCRoute` → `GrpcRoute`
  - `HTTPRoute` → `HttpRoute`
  - `TLSRoute` → `TlsRoute`
  - `TCPRoute` → `TcpRoute` (experimental)
  - `UDPRoute` → `UdpRoute` (experimental)
- **Experimental APIs require the `experimental` feature** ([#242]).
  `gateway_api::apis::experimental` was public even when the feature was off.
  It is now compiled only when `experimental` is enabled; add
  `features = ["experimental"]` if you use it.
- **kube default features are disabled** ([#244]). `gateway-api` now enables
  only kube's `derive` feature. If you relied on `gateway-api` to pull in kube's
  client or TLS features, enable them on your own `kube` dependency.
- **Dependency bumps** ([#241]).
  - `kube` 3.1 → 4.2
  - `k8s-openapi` 0.27 → 0.28
- **MSRV declared as Rust 1.92** (`rust-version`).

### Fixes and improvements

- Removed the unused `serde_yaml` dependency.
- docs.rs now builds with `experimental` enabled and marks feature-gated items.
  The crate docs now describe the `experimental` feature.
- Fixed the crate `homepage` URL.

[Rust API naming guidelines]: https://rust-lang.github.io/api-guidelines/naming.html
[#226]: https://github.com/kube-rs/gateway-api-rs/pull/226
[#241]: https://github.com/kube-rs/gateway-api-rs/pull/241
[#242]: https://github.com/kube-rs/gateway-api-rs/pull/242
[#244]: https://github.com/kube-rs/gateway-api-rs/pull/244
