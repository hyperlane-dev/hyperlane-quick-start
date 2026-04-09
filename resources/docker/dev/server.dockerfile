FROM rust:1.93-bookworm

RUN apt-get update -yqq && apt-get install -yqq cmake g++ binutils lld

WORKDIR /hyperlane-quick-start

COPY . .

RUN cargo build && \
    cp -f /hyperlane-quick-start/target/debug/hyperlane-quick-start /hyperlane-quick-start/hyperlane-quick-start && \
    rm -rf /hyperlane-quick-start/target

EXPOSE 60000

CMD ["/hyperlane-quick-start/hyperlane-quick-start"]
