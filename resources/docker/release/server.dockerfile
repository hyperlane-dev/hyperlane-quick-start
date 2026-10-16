# sea-orm 2.0 and sqlx 0.9 declare rust-version 1.94, so a 1.93 base refuses to
# resolve them and the image build dies partway through with
# "rustc 1.93.1 is not supported by the following packages". The pin matches the
# toolchain CI uses, which is stable rather than a fixed release.
ARG RUST_VERSION=1.99
FROM rust:${RUST_VERSION}-bookworm

RUN apt-get update -yqq && apt-get install -yqq cmake g++ binutils lld

WORKDIR /hyperlane-quick-start

COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN cargo install wasm-bindgen-cli --locked && \
    cargo install wasm-pack --locked

RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release --target x86_64-unknown-linux-gnu && \
    cp -f /hyperlane-quick-start/target/x86_64-unknown-linux-gnu/release/hyperlane-quick-start /hyperlane-quick-start/hyperlane-quick-start

EXPOSE 65002

CMD ["/hyperlane-quick-start/hyperlane-quick-start"]
