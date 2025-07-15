# Etapa de build
FROM rust:1.88-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copia manifestos para cache de dependências
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release

# Agora copia o código real
COPY . .
RUN cargo build --release

# Strip para reduzir tamanho do binário
RUN strip target/release/ld-tr

# Etapa de execução ultra leve
FROM gcr.io/distroless/cc

WORKDIR /app
COPY --from=builder /app/target/release/ld-tr .

# Variáveis de ambiente
ENV PAYMENT_PROCESSOR_DEFAULT=http://localhost:8001
ENV PAYMENT_PROCESSOR_FALLBACK=http://localhost:8002
ENV REDIS=redis://redis:6379/

ENTRYPOINT ["/app/ld-tr"]
