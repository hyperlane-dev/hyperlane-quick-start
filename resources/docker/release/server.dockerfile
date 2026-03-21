FROM rust:1.93-bookworm

RUN sed -i 's/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list.d/debian.sources && \
    apt-get update -yqq && \
    apt-get install -yqq cmake g++ binutils lld

WORKDIR /hyperlane-quick-start

COPY . .

RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release --target x86_64-unknown-linux-gnu

EXPOSE 65002

CMD ["/hyperlane-quick-start/target/x86_64-unknown-linux-gnu/release/hyperlane-quick-start"]
