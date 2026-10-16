# This branch's lockfile pins sea-orm 1.1.20 and sqlx 0.8.6, which resolve
# under 1.93; the playground branch moved to 1.99 because its lockfile requires
# it. The two branches are allowed to differ here.
ARG TARGETPLATFORM
ARG RUST_VERSION=1.93
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
