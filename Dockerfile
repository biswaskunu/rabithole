FROM rust:1.75 AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy cargo files
COPY Cargo.toml Cargo.lock ./

# Build deps (cached layer)
RUN cargo fetch || true

# Copy source code
COPY src ./src

# Build binary
RUN cargo build --release -p rabithole

# Final stage - minimal runtime
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/rabithole ./

# Run the app
ENV RUST_LOG=info
CMD ["./rabithole"]