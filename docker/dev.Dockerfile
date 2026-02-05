FROM rust:1.84-slim

WORKDIR /workspace

RUN rustup component add rustfmt clippy && \
    cargo install cargo-kani --locked

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

CMD ["/bin/bash"]
