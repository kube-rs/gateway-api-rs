# Development

## Code Generation

API bindings are auto-generated from upstream Gateway API CRDs using [Kopium].

Install `kopium`:

```console
$ cargo install kopium --version 0.24.1
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
The generator also vendors the upstream CRD YAML it reads into
`gateway-api/tests/fixtures/crds/<channel>/`, which the integration tests
install and `tests/schema.rs` compares the derived schemas against.

[Kopium]:https://github.com/kube-rs/kopium

## Testing

```console
$ make test.unit                         # Unit tests
$ make test.unit.experimental            # Unit tests with the experimental feature
$ make test.integration                  # Integration tests
$ make test.integration.experimental     # Integration tests with the experimental feature
$ make test.all                          # All of the above
```

Integration tests require Docker and [Kind](https://kind.sigs.k8s.io/) installed locally.
