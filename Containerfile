#syntax=docker/dockerfile-upstream:1.4


# Base image for builds and cache
ARG RUSTUP_TOOLCHAIN
FROM docker.io/lukemathwalker/cargo-chef:latest-rust-${RUSTUP_TOOLCHAIN}-bookworm as cargo-chef
WORKDIR /build


# Stores source cache and cargo chef recipe
FROM cargo-chef as chef-planner
WORKDIR /src
COPY . .

# Select only the essential files for copying into next steps
# so that changes to miscellaneous files don't trigger a new cargo-chef cook.
# Beware that .dockerignore filters files before they get here.

RUN find . \( \
    -name "*.rs" -or \
    -name "*.toml" -or \
    -name "Cargo.lock" -or \
    -name "*.sql" -or \
    -name "README.md" -or \
    # Used for local TLS testing, as described in admin/README.md
    -name "*.pem" \
    \) -type f -exec install -D \{\} /build/\{\} \;
WORKDIR /build
RUN cargo chef prepare --recipe-path /recipe.json
# TODO upstream: Reduce the cooking by allowing multiple --bin args to prepare, or like this https://github.com/LukeMathWalker/cargo-chef/issues/181


# Builds crate according to cargo chef recipe.
# This step is skipped if the recipe is unchanged from previous build (no dependencies changed).
FROM cargo-chef AS chef-builder
ARG CARGO_PROFILE
COPY --from=chef-planner /recipe.json /
# https://i.imgflip.com/2/74bvex.jpg
RUN cargo chef cook \
    --all-features \
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi) \
    --recipe-path /recipe.json
COPY --from=chef-planner /build .
# Building all at once to share build artifacts in the "cook" layer
RUN cargo build \
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi) \
    --bin cyndra-auth \
    # --bin cyndra-builder \
    --bin cyndra-deployer \
    --bin cyndra-gateway \
    --bin cyndra-logger \
    --bin cyndra-provisioner \
    --bin cyndra-resource-recorder \
    --bin cyndra-next -F next


####### Helper step

FROM docker.io/library/debian:bookworm-20230904-slim AS bookworm-20230904-slim-plus
RUN apt update && apt install -y curl ca-certificates; rm -rf /var/lib/apt/lists/*

####### Targets for each crate

#### AUTH
FROM bookworm-20230904-slim-plus AS cyndra-auth
ARG cyndra_SERVICE_VERSION
ENV cyndra_SERVICE_VERSION=${cyndra_SERVICE_VERSION}
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/cyndra-auth /usr/local/bin
ENTRYPOINT ["/usr/local/bin/cyndra-auth"]
FROM cyndra-auth AS cyndra-auth-dev


#### BUILDER
# ARG RUSTUP_TOOLCHAIN
# FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-bookworm AS cyndra-builder
# ARG cyndra_SERVICE_VERSION
# ENV cyndra_SERVICE_VERSION=${cyndra_SERVICE_VERSION}
# ARG CARGO_PROFILE
# ARG prepare_args
# COPY builder/prepare.sh /prepare.sh
# RUN /prepare.sh "${prepare_args}"
# COPY --from=chef-builder /build/target/${CARGO_PROFILE}/cyndra-builder /usr/local/bin
# ENTRYPOINT ["/usr/local/bin/cyndra-builder"]
# FROM cyndra-builder AS cyndra-builder-dev


#### DEPLOYER
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-bookworm AS cyndra-deployer
ARG cyndra_SERVICE_VERSION
ENV cyndra_SERVICE_VERSION=${cyndra_SERVICE_VERSION}
ARG CARGO_PROFILE
ARG prepare_args
# Fixes some dependencies compiled with incompatible versions of rustc
ARG RUSTUP_TOOLCHAIN
ENV RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}
COPY gateway/ulid0.so /usr/lib/
COPY gateway/ulid0_aarch64.so /usr/lib/
ENV LD_LIBRARY_PATH=/usr/lib/
ARG TARGETPLATFORM
RUN for target_platform in "linux/arm64" "linux/arm64/v8"; do \
    if [ "$TARGETPLATFORM" = "$target_platform" ]; then \
      mv /usr/lib/ulid0_aarch64.so /usr/lib/ulid0.so; fi; done
# Used as env variable in prepare script
ARG cyndra_ENV
# Easy way to check if you are running in Cyndra's container
ARG cyndra=true
ENV cyndra=${cyndra}
COPY deployer/prepare.sh /prepare.sh
COPY scripts/apply-patches.sh /scripts/apply-patches.sh
COPY scripts/patches.toml /scripts/patches.toml
RUN /prepare.sh "${prepare_args}"
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/cyndra-deployer /usr/local/bin
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/cyndra-next /usr/local/cargo/bin
ENTRYPOINT ["/usr/local/bin/cyndra-deployer"]
FROM cyndra-deployer AS cyndra-deployer-dev
# Source code needed for compiling local deploys with [patch.crates-io]
COPY --from=chef-planner /build /usr/src/cyndra/


#### GATEWAY
FROM bookworm-20230904-slim-plus AS cyndra-gateway
ARG cyndra_SERVICE_VERSION
ENV cyndra_SERVICE_VERSION=${cyndra_SERVICE_VERSION}
ARG CARGO_PROFILE
COPY gateway/ulid0.so /usr/lib/
COPY gateway/ulid0_aarch64.so /usr/lib/
ENV LD_LIBRARY_PATH=/usr/lib/
ARG TARGETPLATFORM
RUN for target_platform in "linux/arm64" "linux/arm64/v8"; do \
    if [ "$TARGETPLATFORM" = "$target_platform" ]; then \
      mv /usr/lib/ulid0_aarch64.so /usr/lib/ulid0.so; fi; done
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/cyndra-gateway /usr/local/bin
ENTRYPOINT ["/usr/local/bin/cyndra-gateway"]
FROM cyndra-gateway AS cyndra-gateway-dev
# For testing certificates locally
COPY --from=chef-planner /build/*.pem /usr/src/cyndra/


#### LOGGER
FROM docker.io/library/debian:bookworm-20230904-slim AS cyndra-logger
ARG cyndra_SERVICE_VERSION
ENV cyndra_SERVICE_VERSION=${cyndra_SERVICE_VERSION}
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/cyndra-logger /usr/local/bin
ENTRYPOINT ["/usr/local/bin/cyndra-logger"]
FROM cyndra-logger AS cyndra-logger-dev


#### PROVISIONER
ARG RUSTUP_TOOLCHAIN
FROM bookworm-20230904-slim-plus AS cyndra-provisioner
ARG cyndra_SERVICE_VERSION
ENV cyndra_SERVICE_VERSION=${cyndra_SERVICE_VERSION}
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/cyndra-provisioner /usr/local/bin
ENTRYPOINT ["/usr/local/bin/cyndra-provisioner"]
FROM cyndra-provisioner AS cyndra-provisioner-dev


#### RESOURCE RECORDER
FROM docker.io/library/debian:bookworm-20230904-slim AS cyndra-resource-recorder
ARG cyndra_SERVICE_VERSION
ENV cyndra_SERVICE_VERSION=${cyndra_SERVICE_VERSION}
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/cyndra-resource-recorder /usr/local/bin
ENTRYPOINT ["/usr/local/bin/cyndra-resource-recorder"]
FROM cyndra-resource-recorder AS cyndra-resource-recorder-dev
