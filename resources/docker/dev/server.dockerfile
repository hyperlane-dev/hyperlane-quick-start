FROM rust:1.93-bookworm

RUN apt-get update -yqq && apt-get install -yqq cmake g++ binutils lld

WORKDIR /hyperlane-quick-start

COPY . .

RUN cargo install wasm-bindgen-cli --locked && \
    cargo install wasm-pack --locked

RUN cargo build && \
    cp -f /hyperlane-quick-start/target/debug/hyperlane-quick-start /hyperlane-quick-start/hyperlane-quick-start

EXPOSE 80

CMD ["/hyperlane-quick-start/hyperlane-quick-start"]
