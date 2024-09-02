#syntax=docker/dockerfile-upstream:1.4.0-rc1
FROM rust:1.63.0-buster as cyndra-build
RUN apt-get update &&\
    apt-get install -y curl protobuf-compiler
RUN cargo install cargo-chef
WORKDIR /build

FROM cyndra-build as cache
WORKDIR /src
COPY . .
RUN find ${SRC_CRATES} \( -name "*.proto" -or -name "*.rs" -or -name "*.toml" -or -name "README.md" -or -name "*.sql" \) -type f -exec install -D \{\} /build/\{\} \;

FROM cyndra-build AS planner
COPY --from=cache /build .
RUN cargo chef prepare --recipe-path recipe.json

FROM cyndra-build AS builder
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY --from=cache /build .
ARG crate
RUN cargo build --bin ${crate}

FROM rust:1.63.0-buster as cyndra-common
RUN apt-get update &&\
    apt-get install -y curl
RUN rustup component add rust-src
COPY --from=cache /build/ /usr/src/cyndra/

FROM cyndra-common
ARG crate
SHELL ["/bin/bash", "-c"]
RUN mkdir -p $CARGO_HOME; \
echo $'[patch.crates-io] \n\
cyndra-service = { path = "/usr/src/cyndra/service" } \n\
cyndra-aws-rds = { path = "/usr/src/cyndra/resources/aws-rds" } \n\
cyndra-persist = { path = "/usr/src/cyndra/resources/persist" } \n\
cyndra-shared-db = { path = "/usr/src/cyndra/resources/shared-db" } \n\
cyndra-secrets = { path = "/usr/src/cyndra/resources/secrets" }' > $CARGO_HOME/config.toml
COPY --from=builder /build/target/debug/${crate} /usr/local/bin/service
ENTRYPOINT ["/usr/local/bin/service"]
