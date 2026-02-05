FROM rust:1.84-slim as builder

WORKDIR /app
COPY . .

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    cargo build -p hexen-workers --release

FROM debian:stable-slim
WORKDIR /srv/worker

COPY --from=builder /app/target/release/hexen-workers /usr/local/bin/hexen-workers

ENV RUST_LOG=info
ENTRYPOINT ["/usr/local/bin/hexen-workers"]
