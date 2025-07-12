# Etapa 1: Builder
FROM rust:1.88.0-slim-bullseye AS builder

# Instala dependências para compilar crates como openssl, diesel, etc.
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    jq \
    curl \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar manifestos para aproveitar cache de dependências
COPY Cargo.toml Cargo.lock ./

# Compilar dependências para cache
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -r src

# Copiar todo o código restante
COPY . .

# Compilar o binário final
RUN cargo build --release

# Detectar dinamicamente o nome do binário
RUN BIN_NAME=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].name') && \
    cp target/release/$BIN_NAME /app/app_bin

# Etapa 2: Runtime enxuto
FROM debian:bullseye-slim

# Instalar bibliotecas necessárias para execução do binário
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binário construído
COPY --from=builder /app/app_bin .

# Variáveis de ambiente
ENV LOAD_BALANCE=http://127.0.0.1:8081

# Comando padrão
CMD ["./app_bin"]
