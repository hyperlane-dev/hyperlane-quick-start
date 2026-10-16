# sea-orm 2.0 and sqlx 0.9 declare rust-version 1.94, so a 1.93 base refuses to
# resolve them and the image build dies partway through with
# "rustc 1.93.1 is not supported by the following packages". The pin matches the
# toolchain CI uses, which is stable rather than a fixed release.
ARG TARGETPLATFORM
ARG RUST_VERSION=1.99
FROM --platform=$TARGETPLATFORM rust:${RUST_VERSION}-bookworm

RUN apt-get update -yqq && apt-get install -yqq cmake g++ binutils lld

WORKDIR /hyperlane-quick-start

COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-bindgen-cli --locked && \
    cargo install wasm-pack --locked

RUN cargo build && \
    cp -f /hyperlane-quick-start/target/debug/hyperlane-quick-start /hyperlane-quick-start/hyperlane-quick-start

# The data directories the dev env file points at. The server refuses to start
# if its log or pid directory is missing, and neither exists in a fresh image.
RUN mkdir -p /hyperlane-quick-start/data/dev/logs /hyperlane-quick-start/data/dev/process

EXPOSE 80

CMD ["/hyperlane-quick-start/hyperlane-quick-start"]
