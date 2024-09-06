## cyndra-next runtime-wrapper

Load and run an .so library that implements `cyndra_service::Service`. 

To load and run, pass the path to the .so file to load as an argument to the cyndra-next binary:

```bash
cargo run -- -f "src/libhello_world.so"
```
