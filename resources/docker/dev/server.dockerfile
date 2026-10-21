# Multi-stage development image for Hyperlane Quick Start.
#
# The layout mirrors the release image: a builder stage that compiles a
# static (musl) binary, and a runtime stage that carries that binary plus
# the resources the server reads. The difference is the profile: this
# image builds the debug profile, because a development container is meant
# to run the code as it is being edited.
#
# Stage 1 compiles a fully static (musl) binary. Stage 2 is the runtime
# image and carries that binary plus the static assets the server serves;
# the build intermediates - the crate registry and the whole target/ tree
# of the server build - never reach the published image.
#
# The musl target is what makes the runtime image possible: a glibc-linked
# binary would need the dynamic loader and the shared libraries it was
# built against, so the "keep only the executable" step would not work.
# A static musl binary also resolves Docker Compose service names, because
# musl's resolver reads /etc/resolv.conf directly instead of going
# through the glibc NSS modules that a static glibc binary cannot load.
#
# This branch also serves the euv playground, which compiles user code to
# WebAssembly at runtime. That needs cargo, rustc with the wasm32 target,
# wasm-pack and wasm-bindgen inside the running image, so the runtime
# stage is based on the Rust image rather than on plain Alpine. The tools
# are taken from the prebuilt musl releases, and the server's own build
# tree is not copied over.
#
# sea-orm 2.0.4 and sqlx 0.9.0 declare rust-version 1.94, and this branch's
# Cargo.lock pins both, so a 1.93 base refuses to resolve them and the build
# stops with "rustc 1.93.1 is not supported by the following packages". The pin
# matches the toolchain CI uses, which is stable rather than a fixed release.
ARG TARGETPLATFORM
ARG RUST_VERSION=1.99
ARG ALPINE_VERSION=3.22
ARG WASM_PACK_VERSION=0.15.0
ARG WASM_BINDGEN_VERSION=0.2.129
FROM --platform=$TARGETPLATFORM rust:${RUST_VERSION}-alpine AS builder

# A build argument is only visible in the stage that declares it, so the
# version of the wasm tools is repeated here for the builder stage; without
# it the download of wasm-pack would look for a release without a version.
ARG WASM_PACK_VERSION
ARG WASM_BINDGEN_VERSION

# Alpine's apk is the build toolchain for the musl target: build-base
# provides the C compiler and linker that the aws-lc-sys, ring and sqlx
# build scripts drive, and cmake is required by aws-lc-sys.
RUN apk add --no-cache build-base cmake perl linux-headers file curl

WORKDIR /hyperlane-quick-start

# The dependency manifests are copied before the sources, so the registry
# download layer is reused whenever only the application code changes.
# The crate roots are copied too, because `cargo fetch` refuses a manifest
# that names no target; they are the real files, so the layer stays valid
# when the sources below replace them.
COPY Cargo.toml Cargo.lock ./
COPY application/Cargo.toml application/Cargo.toml
COPY application/lib.rs application/lib.rs
COPY bootstrap/Cargo.toml bootstrap/Cargo.toml
COPY bootstrap/lib.rs bootstrap/lib.rs
COPY config/Cargo.toml config/Cargo.toml
COPY config/lib.rs config/lib.rs
COPY plugin/Cargo.toml plugin/Cargo.toml
COPY plugin/lib.rs plugin/lib.rs
COPY resources/Cargo.toml resources/Cargo.toml
COPY resources/lib.rs resources/lib.rs
COPY src/main.rs src/main.rs
RUN cargo fetch

COPY . .

# The wasm target is needed to compile the playground's user code to
# WebAssembly, and the frontend pages of the repository to WebAssembly.
RUN rustup target add wasm32-unknown-unknown

# wasm-pack compiles the pages of the frontend workspace. It is taken from
# the prebuilt musl release, so it does not need a Rust build of its own in
# the image.
RUN set -eux; \
    arch="$(uname -m)"; \
    case "$arch" in \
        x86_64) wasm_arch="x86_64" ;; \
        aarch64) wasm_arch="aarch64" ;; \
        *) echo "unsupported architecture: $arch" >&2; exit 1 ;; \
    esac; \
    cd /tmp; \
    curl -sSfL -o wasm-pack.tar.gz \
      "https://github.com/rustwasm/wasm-pack/releases/download/v${WASM_PACK_VERSION}/wasm-pack-v${WASM_PACK_VERSION}-${wasm_arch}-unknown-linux-musl.tar.gz"; \
    tar xzf wasm-pack.tar.gz; \
    cp -f "wasm-pack-v${WASM_PACK_VERSION}-${wasm_arch}-unknown-linux-musl/wasm-pack" /usr/local/bin/wasm-pack; \
    chmod 0755 /usr/local/bin/wasm-pack; \
    rm -rf /tmp/wasm-pack.tar.gz /tmp/wasm-pack-v*

# The wasm-bindgen command line interface is taken from the prebuilt musl
# release as well. wasm-pack installs it on its own when it does not find it
# in the path, and that install needs a release of the architecture of the
# builder, which the image does not always have.
RUN set -eux; \
    arch="$(uname -m)"; \
    case "$arch" in \
        x86_64) wasm_arch="x86_64" ;; \
        aarch64) wasm_arch="aarch64" ;; \
        *) echo "unsupported architecture: $arch" >&2; exit 1 ;; \
    esac; \
    cd /tmp; \
    curl -sSfL -o wasm-bindgen.tar.gz \
      "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-${wasm_arch}-unknown-linux-musl.tar.gz"; \
    tar xzf wasm-bindgen.tar.gz; \
    cp -f "wasm-bindgen-${WASM_BINDGEN_VERSION}-${wasm_arch}-unknown-linux-musl/wasm-bindgen" /usr/local/bin/wasm-bindgen; \
    chmod 0755 /usr/local/bin/wasm-bindgen; \
    rm -rf /tmp/wasm-bindgen.tar.gz /tmp/wasm-bindgen-${WASM_BINDGEN_VERSION}-*

# The euv command line interface builds the pages of the frontend workspace.
# The build script of the server package drives it, so the pages are part of
# the build of the server itself.
RUN cargo install euv-cli --version 0.31.3 --locked

# The build itself runs on the Alpine (musl) toolchain, so the produced
# binary is statically linked against musl and needs no shared library
# at runtime. The `file` check fails the build when the toolchain ever
# stops producing a static binary, instead of shipping one that would
# only work inside this base image.
RUN cargo build && \
    test -x /hyperlane-quick-start/target/debug/hyperlane-quick-start && \
    file /hyperlane-quick-start/target/debug/hyperlane-quick-start | grep -Eq "static-pie linked|statically linked" && \
    cp -f /hyperlane-quick-start/target/debug/hyperlane-quick-start /hyperlane-quick-start/hyperlane-quick-start

# The runtime image. It is based on the Rust image because the euv
# playground compiles user code at runtime and therefore needs cargo and
# rustc with the wasm32 target, plus wasm-pack and wasm-bindgen. The two
# wasm tools are installed from their prebuilt musl releases, so they do
# not need a Rust build of their own in the image.
FROM --platform=$TARGETPLATFORM rust:${RUST_VERSION}-alpine AS runtime

# The build arguments are scoped to the stage that declares them, so the
# two wasm tool versions are repeated here for the runtime stage.
ARG WASM_PACK_VERSION
ARG WASM_BINDGEN_VERSION

RUN apk add --no-cache build-base cmake perl linux-headers file curl

# The wasm32 target is the one the playground compiles to.
RUN rustup target add wasm32-unknown-unknown

# wasm-pack and wasm-bindgen are installed from their prebuilt musl
# archives, so the image does not have to compile them from source.
RUN set -eux; \
    arch="$(uname -m)"; \
    case "$arch" in \
        x86_64) wasm_arch="x86_64" ;; \
        aarch64) wasm_arch="aarch64" ;; \
        *) echo "unsupported architecture: $arch" >&2; exit 1 ;; \
    esac; \
    cd /tmp; \
    curl -sSfL -o wasm-pack.tar.gz \
      "https://github.com/rustwasm/wasm-pack/releases/download/v${WASM_PACK_VERSION}/wasm-pack-v${WASM_PACK_VERSION}-${wasm_arch}-unknown-linux-musl.tar.gz"; \
    tar xzf wasm-pack.tar.gz; \
    cp -f "wasm-pack-v${WASM_PACK_VERSION}-${wasm_arch}-unknown-linux-musl/wasm-pack" /usr/local/bin/wasm-pack; \
    chmod 0755 /usr/local/bin/wasm-pack; \
    curl -sSfL -o wasm-bindgen.tar.gz \
      "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-${wasm_arch}-unknown-linux-musl.tar.gz"; \
    tar xzf wasm-bindgen.tar.gz; \
    cp -f "wasm-bindgen-${WASM_BINDGEN_VERSION}-${wasm_arch}-unknown-linux-musl/wasm-bindgen" /usr/local/bin/wasm-bindgen; \
    chmod 0755 /usr/local/bin/wasm-bindgen; \
    rm -rf /tmp/wasm-pack.tar.gz /tmp/wasm-bindgen.tar.gz /tmp/wasm-pack-v* /tmp/wasm-bindgen-*

WORKDIR /hyperlane-quick-start

# The server reads its configuration from ./resources/env, its compose
# topology from ./resources/docker, and the browser assets from
# ./resources/static. The resources directory is copied as a whole because
# the branch that only exposes the API has no static assets at all, and a
# COPY of a missing path fails the build.
COPY resources ./resources
RUN mkdir -p ./resources/static

# The browser assets of the pages are built by the build script of the server
# package, so they are taken from the builder stage instead of the checkout:
# the build writes them into the static resource directory of the source tree.
COPY --from=builder /hyperlane-quick-start/resources/static ./resources/static

# The single build output. The server's own build tree - the crate
# registry and the whole target/ directory - is not copied over; only the
# linked executable is.
COPY --from=builder /hyperlane-quick-start/hyperlane-quick-start /hyperlane-quick-start/hyperlane-quick-start

# The data directories the dev env file points at. The server refuses to
# start if its log or pid directory is missing, and neither exists in a fresh
# image.
RUN mkdir -p /hyperlane-quick-start/data/dev/logs /hyperlane-quick-start/data/dev/process

EXPOSE 80

CMD ["/hyperlane-quick-start/hyperlane-quick-start"]
