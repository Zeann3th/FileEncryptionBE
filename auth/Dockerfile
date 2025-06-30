FROM rust:1.88 AS builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
        tzdata && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/auth /usr/local/bin/auth

EXPOSE 8000

CMD ["auth"]