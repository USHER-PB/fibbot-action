# Use a valid Rust nightly image with Alpine Linux
FROM rustlang/rust:nightly-alpine3.19

# Install system dependencies
RUN apk add --no-cache \
    openssl-dev \
    perl \
    pkgconfig \
    musl-dev \
    ca-certificates \
    make \
    gcc \
    libc-dev \
    openssl \
    openssl-libs-static

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the project
RUN cargo build --release \
    --features "openssl-sys/vendored" \
    --target x86_64-unknown-linux-musl

# Optionally, you can specify the entry point if your project produces a binary
# ENTRYPOINT ["./target/release/your_binary_name"]