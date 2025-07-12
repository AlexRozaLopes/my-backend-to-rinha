# Stage 1: Build
FROM rust:1.80-slim-bullseye AS builder

WORKDIR /app

# Instalar dependências para build
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Copiar arquivos de configuração e instalar dependências
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copiar o código-fonte e compilar
COPY src ./src
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

WORKDIR /app

# Instalar dependências de runtime
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Criar usuário não-root
RUN useradd -m myuser
USER myuser

# Copiar o binário compilado
COPY --from=builder /app/target/release/b-rs .

# Expor a porta
EXPOSE 9999

# Configurar variável de ambiente (opcional, pode ser definida em tempo de execução)
ENV LOAD_BALANCE=http://127.0.0.1:8081

# Comando padrão
CMD ["./b-rs"]