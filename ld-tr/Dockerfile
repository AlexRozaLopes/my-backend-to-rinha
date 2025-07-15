# Etapa 1: Builder
FROM rust:1.88.0-slim-bullseye AS builder

WORKDIR /app

# Instala dependências necessárias para compilar crates como openssl, reqwest, etc.
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists

# Copia os arquivos do projeto
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Compila o projeto em modo release
RUN cargo build --release

# Etapa 2: Imagem final
FROM debian:bullseye-slim

# Instala certificados raiz do sistema
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copia o binário da etapa de build
COPY --from=builder /app/target/release/ld-tr .

ENV PAYMENT_PROCESSOR_DEFAULT=http://localhost:8001
ENV PAYMENT_PROCESSOR_FALLBACK=http://localhost:8002
ENV REDIS=redis://redis:6379/

# Comando padrão
CMD ["./ld-tr"]
