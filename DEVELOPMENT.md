# Development

## Code Generation

API bindings are auto-generated from upstream Gateway API CRDs using [Kopium].

Install `kopium`:

```console
$ cargo install kopium --version 0.21.1
```

Run the generator:

```console
$ make generate
```

Or with a specific version:

```console
$ make generate GATEWAY_API_VERSION=v1.5.1
```

Generated code lives in `gateway-api/src/apis/` — do not edit it by hand.

[Kopium]:https://github.com/kube-rs/kopium

## Testing

```console
$ cargo test -v -- --nocapture              # Unit tests
$ cargo test -v -- --nocapture --ignored    # Integration tests (requires Kind cluster)
$ make test.all                             # Both
```

Integration tests require Docker and [Kind](https://kind.sigs.k8s.io/) installed locally.
