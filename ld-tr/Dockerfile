FROM rust:1.88-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*


WORKDIR /app
COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo 'fn main() {}' > src/main.rs

RUN cargo build --release

COPY src src

RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copia o binário da etapa de build
COPY --from=builder /app/target/release/ld-tr .

ENV PAYMENT_PROCESSOR_DEFAULT=http://localhost:8001
ENV PAYMENT_PROCESSOR_FALLBACK=http://localhost:8002
ENV REDIS=redis://redis:6379/

# Comando padrão
CMD ["./ld-tr"]
