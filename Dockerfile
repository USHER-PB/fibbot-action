# Use the official Rust image as the build stage
FROM rust:1.85 as builder

# Set the working directory
WORKDIR /usr/src/fibbot

# Install musl-tools and other necessary packages
RUN apt-get update && apt-get install -y musl-tools clang pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Add the musl target
RUN rustup target add x86_64-unknown-linux-musl

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the actual source code
COPY . .

# Build the actual application statically
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use a minimal base image
FROM scratch

# Copy the statically linked binary
COPY --from=builder /usr/src/fibbot/target/x86_64-unknown-linux-musl/release/fibbot /usr/local/bin/fibbot

# Set the entrypoint for the container
ENTRYPOINT ["/usr/local/bin/fibbot"]