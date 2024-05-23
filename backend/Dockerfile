FROM rust:slim-buster as builder

WORKDIR /home/lembas

COPY . .

RUN cargo build --release

# Create a separate image which will only contain the built binary.
FROM debian:buster-slim

COPY --from=builder /home/lembas/target/release/lembas /usr/local/bin/lembas

CMD ["lembas"]
