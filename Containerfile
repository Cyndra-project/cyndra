#syntax=docker/dockerfile-upstream:1.4


# Base image for builds and cache
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-buster as cyndra-build
RUN cargo install cargo-chef --locked
WORKDIR /build


# Stores source cache
FROM cyndra-build as cache
ARG CARGO_PROFILE
WORKDIR /src
COPY . .
RUN find ${SRC_CRATES} \( -name "*.proto" -or -name "*.rs" -or -name "*.toml" -or -name "Cargo.lock" -or -name "README.md" -or -name "*.sql" \) -type f -exec install -D \{\} /build/\{\} \;
# This is used to carry over in the docker images any *.pem files from cyndra root directory,
# to be used for TLS testing, as described here in the admin README.md.
RUN [ "$CARGO_PROFILE" != "release" ] && \
    find ${SRC_CRATES} -name "*.pem" -type f -exec install -D \{\} /build/\{\} \;


# Stores cargo chef recipe
FROM cyndra-build AS planner
COPY --from=cache /build .
RUN cargo chef prepare --recipe-path recipe.json


# Builds crate according to cargo chef recipe
FROM cyndra-build AS builder
ARG CARGO_PROFILE
ARG folder
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook \
    # if CARGO_PROFILE is release, pass --release, else use default debug profile
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi) \
    --recipe-path recipe.json
COPY --from=cache /build .
RUN cargo build --bin cyndra-${folder} \
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi)


# The final image for this "cyndra-..." crate
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-buster as cyndra-crate
ARG folder
ARG prepare_args
# used as env variable in prepare script
ARG PROD
ARG CARGO_PROFILE
ARG RUSTUP_TOOLCHAIN
ENV RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}

COPY ${folder}/prepare.sh /prepare.sh
RUN /prepare.sh "${prepare_args}"

COPY --from=cache /build /usr/src/cyndra/

# Any prepare steps that depend on the COPY from src cache
RUN /prepare.sh --after-src "${prepare_args}"

COPY --from=builder /build/target/${CARGO_PROFILE}/cyndra-${folder} /usr/local/bin/service
ENTRYPOINT ["/usr/local/bin/service"]
