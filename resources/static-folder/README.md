# Cyndra Static Folder

**This plugin is deprecated.**

Your binaries now execute in the workspace root, meaning paths can be declared with strings or paths as per usual.

Using the macro still works for backward compatibility:

``` rust
#[cyndra_runtime::main]
async fn app(
    #[cyndra_static_folder::StaticFolder] static_folder: PathBuf,
) -> __ { ... }
```

``` rust
#[cyndra_runtime::main]
async fn app(
    #[cyndra_static_folder::StaticFolder(folder = "public")] public_folder: PathBuf,
) -> __ { ... }
```
