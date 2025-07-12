# Stage 1: Build
FROM rust:slim-bullseye AS builder

WORKDIR /app

# Instalar dependências para build
RUN apt-get update &&  apt install -y libpq-dev

# Copiar o código-fonte para o container
COPY . .

# Compilar o projeto em modo release
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

ENV PAYMENT_PROCESSOR_DEFAULT=http://localhost:8001
ENV PAYMENT_PROCESSOR_FALLBACK=http://localhost:8002

WORKDIR /app

# Instalar dependências de runtime
RUN apt-get update &&  apt install -y libpq-dev
# Copiar apenas o binário compilado do estágio anterior
COPY --from=builder /app/target/release/ld-tr .

# Configurar o comando padrão
CMD ["./ld-tr"]
