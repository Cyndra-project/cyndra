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
cyndra-secrets = { path = "/usr/src/cyndra/resources/secrets" }
cyndra-static-folder = { path = "/usr/src/cyndra/resources/static-folder" }' > $CARGO_HOME/config.toml

# Make future crates requests to our own mirror
echo '
[source.cyndra-crates-io-mirror]
registry = "http://panamax:8080/git/crates.io-index"
[source.crates-io]
replace-with = "cyndra-crates-io-mirror"' >> $CARGO_HOME/config.toml

# Prefetch crates.io index from our mirror
# TODO: restore when we know how to prefetch from our mirror
# cd /usr/src/cyndra/service
# cargo fetch
