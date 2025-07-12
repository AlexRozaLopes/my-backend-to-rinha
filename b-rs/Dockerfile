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

ENV LOAD_BALANCE=http://127.0.0.1:8081

WORKDIR /app

EXPOSE 9999

# Instalar dependências de runtime
RUN apt-get update &&  apt install -y libpq-dev
# Copiar apenas o binário compilado do estágio anterior
COPY --from=builder /app/target/release/b-rs .

# Configurar o comando padrão
CMD ["./b-rs"]
