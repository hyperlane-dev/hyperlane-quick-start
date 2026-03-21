FROM rust:1.93-bookworm AS builder

RUN sed -i 's/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list.d/debian.sources && \
    apt-get update -yqq && \
    apt-get install -yqq cmake g++ binutils lld

WORKDIR /hyperlane-quick-start

COPY . .

RUN cargo build

FROM debian:bookworm-slim

RUN sed -i 's/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list.d/debian.sources

WORKDIR /hyperlane-quick-start

COPY --from=builder /hyperlane-quick-start/target/debug/hyperlane-quick-start /hyperlane-quick-start/server

EXPOSE 60000

CMD ["/hyperlane-quick-start/server"]
