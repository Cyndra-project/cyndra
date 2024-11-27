#!/bin/bash

###############################################################################
# This file is used by our common Containerfile incase the container for this #
# service might need some extra preparation steps for its final image         #
###############################################################################

# Stuff that depends on local source files
if [ "$1" = "--after-src" ]; then
    # Install the cyndra-next runtime for cyndra-next services.
    cargo install cyndra-runtime --path "/usr/src/cyndra/runtime" --bin cyndra-next --features next || exit 1

    while getopts "p," o; do
    case $o in
        "p") # if panamax is used, the '-p' parameter is passed
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
    exit 0
fi

# Patch crates to be on same versions
mkdir -p $CARGO_HOME
touch $CARGO_HOME/config.toml
if [[ $PROD != "true" ]]; then
    echo '
    [patch.crates-io]
    cyndra-service = { path = "/usr/src/cyndra/service" }
    cyndra-runtime = { path = "/usr/src/cyndra/runtime" }

    cyndra-aws-rds = { path = "/usr/src/cyndra/resources/aws-rds" }
    cyndra-persist = { path = "/usr/src/cyndra/resources/persist" }
    cyndra-shared-db = { path = "/usr/src/cyndra/resources/shared-db" }
    cyndra-secrets = { path = "/usr/src/cyndra/resources/secrets" }
    cyndra-static-folder = { path = "/usr/src/cyndra/resources/static-folder" }
    cyndra-metadata = { path = "/usr/src/cyndra/resources/metadata" }
    cyndra-turso = { path = "/usr/src/cyndra/resources/turso" }

    cyndra-actix-web = { path = "/usr/src/cyndra/services/cyndra-actix-web" }
    cyndra-axum = { path = "/usr/src/cyndra/services/cyndra-axum" }
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
fi

# Add the wasm32-wasi target
rustup target add wasm32-wasi

# Install common build tools for external crates
# The image should already have these: https://github.com/docker-library/buildpack-deps/blob/65d69325ad741cea6dee20781c1faaab2e003d87/debian/buster/Dockerfile
apt update
apt install -y curl llvm-dev libclang-dev clang cmake

# Install protoc since some users may need it
ARCH="linux-x86_64" && \
VERSION="22.2" && \
curl -OL "https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/protoc-$VERSION-$ARCH.zip" && \
    unzip -o "protoc-$VERSION-$ARCH.zip" bin/protoc "include/*" -d /usr/local && \
    rm -f "protoc-$VERSION-$ARCH.zip"
