# Cyndra Service Info

This plugin allows applications to obtain certain information about their runtime environment.

## Usage

Add `cyndra-service-info` to the dependencies for your service.

You can get this resource using the `cyndra_service_info::CyndraServiceInfo` attribute to get a `ServiceInfo`. This struct will contain information such as the Cyndra service name.

```rust
#[cyndra_runtime::main]
async fn app(
    #[cyndra_service_info::CyndraServiceInfo] service_info: cyndra_service_info::ServiceInfo,
) -> __ { ... }
```

#### Example projects that use `cyndra-service-info`

| Framework | Link                                                                                       |
| --------- | ------------------------------------------------------------------------------------------ |
| Axum      | [axum example](https://github.com/cyndra-hq/cyndra-examples/tree/main/axum/service-info) |
