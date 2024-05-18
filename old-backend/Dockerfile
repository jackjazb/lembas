FROM rust:slim-buster as builder

WORKDIR /home/lembas

COPY . .

ENV SQLX_OFFLINE true
RUN apt update && apt install -y pkg-config libssl-dev
RUN cargo build --release

# Create a separate image which will only contain the built binary.
FROM debian:buster-slim
COPY --from=builder /home/lembas/target/release/lembas /usr/local/bin/lembas

# Install SSL client dependencies and health check tool.
RUN apt update && apt install -y openssl ca-certificates curl
CMD ["lembas"]
