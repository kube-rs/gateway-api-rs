# Changelog

## 0.23.0

### Breaking changes

- **Gateway API v1.7.0** (was v1.5.1). See the [upstream release notes] for
  v1.6.0 and v1.7.0 changes.
- **Condition enums gain variants**: `GatewayConditionType`/`Reason` and
  `ListenerConditionType`/`Reason`.
- **Experimental:** `TcpRoute` and `UdpRoute` are now API version `v1` (was
  `v1alpha2`).
- **Dependency bumps:**
  - `kube` 4.2 → 5.0
  - `k8s-openapi` 0.28 → 0.29

### Features

- **New modules**: `tcproutes`, `udproutes`; experimental: `xbackends`,
  `xtelemetrypolicies`, and `backendtlspolicies`.
- **Bindings for every CRD API version** ([#247]). Submodules for each
  CRD API version, e.g. `gateway_api::httproutes::v1beta1::HttpRoute`.
- **New condition enums**: Route, ListenerSet, ListenerEntry and Policy, plus
  Mesh and Backend in the experimental channel.

[upstream release notes]: https://github.com/kubernetes-sigs/gateway-api/releases
[#247]: https://github.com/kube-rs/gateway-api-rs/pull/247

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
