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
COPY --from=builder /app/target/release/ld-tr .

# Expor a porta
EXPOSE 9999

# Configurar variável de ambiente (opcional, pode ser definida em tempo de execução)
ENV PAYMENT_PROCESSOR_DEFAULT=http://localhost:8001
ENV PAYMENT_PROCESSOR_FALLBACK=http://localhost:8002

# Comando padrão
CMD ["./ld-tr"]