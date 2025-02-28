FROM rust:1.85 as builder

# Install OpenSSL development libraries
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/fibbot

COPY Cargo.toml Cargo.lock ./

COPY src ./src

# Set OPENSSL_DIR if needed
ENV OPENSSL_DIR=/usr/local/ssl

RUN cargo build --release

FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/fibbot/target/release/fibbot /usr/local/bin/fibbot

ENTRYPOINT ["fibbot"]