# cyndra-gateway

## Tests

To run the tests for gateway, follow the steps in [contributing](../CONTRIBUTING.md) to set up your local environment. Then, from the root of the repository, run:

```bash
cyndra_TESTS_RUNTIME_IMAGE=public.ecr.aws/cyndra-dev/deployer:latest cyndra_TESTS_NETWORK=cyndra-dev_user-net cargo test --package cyndra-gateway --all-features -- --nocapture
```
