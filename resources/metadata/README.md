# Cyndra Metadata

This plugin allows applications to obtain certain information about their runtime environment.

## Usage

Add `cyndra-metadata` to the dependencies for your service.

You can get this resource using the `cyndra_metadata::CyndraMetadata` attribute to get a `Metadata`. This struct will contain information such as the Cyndra service name.

```rust
#[cyndra_runtime::main]
async fn app(
    #[cyndra_metadata::CyndraMetadata] metadata: cyndra_metadata::Metadata,
) -> __ { ... }
```

#### Example projects that use `cyndra-metadata`

| Framework | Link                                                                                   |
| --------- | -------------------------------------------------------------------------------------- |
| Axum      | [axum example](https://github.com/cyndra-hq/cyndra-examples/tree/main/axum/metadata) |
