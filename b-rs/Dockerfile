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
COPY --from=builder /app/target/release/b-rs .

# Expõe a porta usada pelo Actix (geralmente 8080)
EXPOSE 9999

# Configurar variável de ambiente (opcional, pode ser definida em tempo de execução)
ENV LOAD_BALANCE=http://127.0.0.1:8081

# Comando padrão
CMD ["./b-rs"]
