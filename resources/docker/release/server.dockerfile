# sea-orm 2.0.4 and sqlx 0.9.0 declare rust-version 1.94, and this branch's
# Cargo.lock pins both, so a 1.93 base refuses to resolve them and the build
# stops with "rustc 1.93.1 is not supported by the following packages". The pin
# matches the toolchain CI uses, which is stable rather than a fixed release.
ARG TARGETPLATFORM
ARG RUST_VERSION=1.99
FROM --platform=$TARGETPLATFORM rust:${RUST_VERSION}-bookworm


ENV CARGO_BUILD_JOBS=1

RUN apt-get update -yqq && apt-get install -yqq cmake g++ binutils lld

WORKDIR /hyperlane-quick-start

COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-bindgen-cli --locked && \
    cargo install wasm-pack --locked

# No --target: the base image matches the requested platform, so a plain build
# produces a binary the container can actually run. Naming x86_64 here instead
# made the build fail off an amd64 host with "can't find crate for `std`",
# because the base had been pulled for the host's own architecture.
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release && \
    cp -f /hyperlane-quick-start/target/release/hyperlane-quick-start /hyperlane-quick-start/hyperlane-quick-start

# The data directories the release env file points at. The server refuses to
# start if its log or pid directory is missing, and neither exists in a fresh
# image.
RUN mkdir -p /hyperlane-quick-start/data/release/logs /hyperlane-quick-start/data/release/process

EXPOSE 65002

CMD ["/hyperlane-quick-start/hyperlane-quick-start"]
