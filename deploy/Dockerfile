FROM rust:1.80 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

# Note: Some shared libraries may need to install the extra-runtime-dependencies.
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/app .
COPY --from=builder /app/config.toml.example ./config.toml

EXPOSE 8000

ENTRYPOINT ["./aphrodite-rs"]

CMD ["serve", "--config", "config.toml"]
