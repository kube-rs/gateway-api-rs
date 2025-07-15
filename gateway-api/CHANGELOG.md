# Changelog

## Next

>[!IMPORTANT] 
Breaking change

### Changed
The structure of APIs has changed to promote the re-use of types in the generated code. The APIs are still generated with Kopium in the first step, but there is a second stage where additional task is executed to reduce and rename the Kopium-generated types.
While with this approach we can significantly reduce the surface of exposed APIs, it is also a breaking change.
See [issue](https://github.com/kube-rs/gateway-api-rs/issues/38) for more context.

## 0.16.0

### Changed
Initial release. All types are generated with Kopium.
