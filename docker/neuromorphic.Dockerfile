FROM rust:1.84-slim

WORKDIR /workspace

RUN apt-get update && \
    apt-get install -y libusb-1.0-0-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .

ENV NEUROMORPHIC_BACKEND=loihi
CMD ["cargo", "run", "-p", "cyberswarm-neurostack", "--bin", "cyberswarm-neurostack"]
