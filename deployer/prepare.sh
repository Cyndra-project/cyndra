#!/usr/bin/env sh

###############################################################################
# This file is used by our common Containerfile incase the container for this #
# service might need some extra preparation steps for its final image         #
###############################################################################

# Patch crates to be on same versions
mkdir -p $CARGO_HOME
if [[ $PROD != "true" ]]; then
    echo '[patch.crates-io]
    cyndra-service = { path = "/usr/src/cyndra/service" }
    cyndra-runtime = { path = "/usr/src/cyndra/runtime" }

    cyndra-aws-rds = { path = "/usr/src/cyndra/resources/aws-rds" }
    cyndra-persist = { path = "/usr/src/cyndra/resources/persist" }
    cyndra-shared-db = { path = "/usr/src/cyndra/resources/shared-db" }
    cyndra-secrets = { path = "/usr/src/cyndra/resources/secrets" }
    cyndra-static-folder = { path = "/usr/src/cyndra/resources/static-folder" }

    cyndra-axum = { path = "/usr/src/cyndra/services/cyndra-axum" }
    cyndra-actix-web = { path = "/usr/src/cyndra/services/cyndra-actix-web" }
    cyndra-next = { path = "/usr/src/cyndra/services/cyndra-next" }
    cyndra-poem = { path = "/usr/src/cyndra/services/cyndra-poem" }
    cyndra-poise = { path = "/usr/src/cyndra/services/cyndra-poise" }
    cyndra-rocket = { path = "/usr/src/cyndra/services/cyndra-rocket" }
    cyndra-salvo = { path = "/usr/src/cyndra/services/cyndra-salvo" }
    cyndra-serenity = { path = "/usr/src/cyndra/services/cyndra-serenity" }
    cyndra-thruster = { path = "/usr/src/cyndra/services/cyndra-thruster" }
    cyndra-tide = { path = "/usr/src/cyndra/services/cyndra-tide" }
    cyndra-tower = { path = "/usr/src/cyndra/services/cyndra-tower" }
    cyndra-warp = { path = "/usr/src/cyndra/services/cyndra-warp" }' > $CARGO_HOME/config.toml
else
    touch $CARGO_HOME/config.toml
fi

# Install protoc since some users may need it
ARCH="linux-x86_64" && \
VERSION="22.2" && \
curl -OL "https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/protoc-$VERSION-$ARCH.zip" && \
    unzip -o "protoc-$VERSION-$ARCH.zip" bin/protoc "include/*" -d /usr/local && \
    rm -f "protoc-$VERSION-$ARCH.zip"

# Add the wasm32-wasi target
rustup target add wasm32-wasi

# Install the cyndra runtime
cargo install cyndra-runtime --path "/usr/src/cyndra/runtime" --bin cyndra-next --features next

while getopts "p," o; do
    case $o in
        "p")
            # Make future crates requests to our own mirror
            echo '
[source.cyndra-crates-io-mirror]
registry = "sparse+http://panamax:8080/index/"
[source.crates-io]
replace-with = "cyndra-crates-io-mirror"' >> $CARGO_HOME/config.toml
            ;;
        *)
            ;;
    esac
done

# Prefetch crates.io index from our mirror
# TODO: restore when we know how to prefetch from our mirror
# cd /usr/src/cyndra/service
# cargo fetch
