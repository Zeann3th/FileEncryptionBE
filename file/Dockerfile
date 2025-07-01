# ---- Build Stage ----
FROM rust:1.88 as builder

WORKDIR /app

# Install protoc (needed to compile .proto files)
RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

COPY . .

# Build and install the binary
RUN cargo install --path .

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install runtime libs needed by the binary
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
        tzdata && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/file /usr/local/bin/file

EXPOSE 8000

CMD ["file"]
