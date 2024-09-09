#!/usr/bin/env sh

###############################################################################
# This file is used by our common Containerfile incase the container for this #
# service might need some extra preparation steps for its final image         #
###############################################################################

# Patch crates to be on same versions
mkdir -p $CARGO_HOME; \
echo '[patch.crates-io]
cyndra-service = { path = "/usr/src/cyndra/service" }
cyndra-aws-rds = { path = "/usr/src/cyndra/resources/aws-rds" }
cyndra-persist = { path = "/usr/src/cyndra/resources/persist" }
cyndra-shared-db = { path = "/usr/src/cyndra/resources/shared-db" }
cyndra-secrets = { path = "/usr/src/cyndra/resources/secrets" }' > $CARGO_HOME/config.toml

# Prefetch crates.io index
cd /usr/src/cyndra/service
cargo fetch
