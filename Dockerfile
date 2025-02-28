FROM rust:1.85 as builder

WORKDIR /usr/src/fibbot

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/fibbot/target/release/fibbot /usr/local/bin/fibbot

ENTRYPOINT ["fibbot"]