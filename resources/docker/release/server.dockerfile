FROM rust:1.93-bookworm AS builder

RUN sed -i 's/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list.d/debian.sources && \
    apt-get update -yqq && \
    apt-get install -yqq cmake g++ binutils lld

WORKDIR /hyperlane-quick-start

COPY . .

RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release --target x86_64-unknown-linux-gnu

FROM debian:bookworm-slim

RUN sed -i 's/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list.d/debian.sources

WORKDIR /hyperlane-quick-start

COPY --from=builder /hyperlane-quick-start/target/x86_64-unknown-linux-gnu/release/hyperlane-quick-start /hyperlane-quick-start/server

EXPOSE 65002

CMD ["/hyperlane-quick-start/server"]
