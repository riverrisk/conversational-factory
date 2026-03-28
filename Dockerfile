FROM rust:1.88-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY services/ services/

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/query-plane /usr/local/bin/query-plane
COPY --from=builder /app/target/release/conversational-gateway /usr/local/bin/conversational-gateway

ENTRYPOINT ["query-plane"]
