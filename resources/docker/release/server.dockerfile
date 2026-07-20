FROM rust:1.93-bookworm

RUN apt-get update -yqq && apt-get install -yqq cmake g++ binutils lld

WORKDIR /hyperlane-quick-start

COPY . .

RUN cargo install wasm-bindgen-cli --locked && \
    cargo install wasm-pack --locked

RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release --target x86_64-unknown-linux-gnu && \
    cp -f /hyperlane-quick-start/target/x86_64-unknown-linux-gnu/release/hyperlane-quick-start /hyperlane-quick-start/hyperlane-quick-start

EXPOSE 65002

CMD ["/hyperlane-quick-start/hyperlane-quick-start"]
