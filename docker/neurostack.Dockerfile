FROM rust:1.84-slim as builder

WORKDIR /app
COPY . .

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    cargo build -p cyberswarm-neurostack --release

FROM debian:stable-slim
WORKDIR /srv/neurostack

COPY --from=builder /app/target/release/cyberswarm-neurostack /usr/local/bin/cyberswarm-neurostack

ENV RUST_LOG=info
EXPOSE 8080 50051

ENTRYPOINT ["/usr/local/bin/cyberswarm-neurostack"]
