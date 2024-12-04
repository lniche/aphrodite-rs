FROM rust:1.82 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/aphrodite-rs .
COPY --from=builder /app/config.toml.example ./config.toml

EXPOSE 8000

CMD ["./aphrodite-rs"]
