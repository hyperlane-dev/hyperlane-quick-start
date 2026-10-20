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
# sea-orm 2.0.4 and sqlx 0.9.0 declare rust-version 1.94, and this branch's
# Cargo.lock pins both, so a 1.93 base refuses to resolve them and the build
# stops with "rustc 1.93.1 is not supported by the following packages". The pin
# matches the toolchain CI uses, which is stable rather than a fixed release.
ARG TARGETPLATFORM
ARG RUST_VERSION=1.99
ARG ALPINE_VERSION=3.22
FROM --platform=$TARGETPLATFORM rust:${RUST_VERSION}-alpine AS builder

# Alpine's apk is the build toolchain for the musl target: build-base
# provides the C compiler and linker that the aws-lc-sys, ring and sqlx
# build scripts drive, and cmake is required by aws-lc-sys.
RUN apk add --no-cache build-base cmake perl linux-headers file

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

# The build itself runs on the Alpine (musl) toolchain, so the produced
# binary is statically linked against musl and needs no shared library
# at runtime. The `file` check fails the build when the toolchain ever
# stops producing a static binary, instead of shipping one that would
# only work inside this base image.
RUN cargo build && \
    test -x /hyperlane-quick-start/target/debug/hyperlane-quick-start && \
    file /hyperlane-quick-start/target/debug/hyperlane-quick-start | grep -Eq "static-pie linked|statically linked" && \
    cp -f /hyperlane-quick-start/target/debug/hyperlane-quick-start /hyperlane-quick-start/hyperlane-quick-start

# The runtime image. `alpine` provides the musl loader the static binary
# does not even need, plus the shell and the CA bundle the server uses
# for its outbound HTTPS calls. This branch does not compile anything at
# runtime, so no Rust toolchain is carried here.
FROM --platform=$TARGETPLATFORM alpine:${ALPINE_VERSION:-3.22}

RUN apk add --no-cache ca-certificates

WORKDIR /hyperlane-quick-start

# The server reads its configuration from ./resources/env, its compose
# topology from ./resources/docker, and the browser assets from
# ./resources/static. The resources directory is copied as a whole because
# the branch that only exposes the API has no static assets at all, and a
# COPY of a missing path fails the build.
COPY resources ./resources
RUN mkdir -p ./resources/static

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
