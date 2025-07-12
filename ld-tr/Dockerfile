# Stage 1: Builder com musl e Alpine
FROM rust:1.88.0-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig build-base jq

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copiar arquivos para cache de dependências
COPY Cargo.toml Cargo.lock ./

# Build das dependências
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -r src

# Copiar o resto do projeto
COPY . .

# Build final do binário
RUN cargo build --release --target x86_64-unknown-linux-musl

# Detecta dinamicamente o nome do binário com cargo metadata
RUN BIN_NAME=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].name') && \
    cp target/x86_64-unknown-linux-musl/release/$BIN_NAME /app/app_bin

# Stage 2: Runtime super leve
FROM alpine:latest

WORKDIR /app

# Copiar o binário construído dinamicamente
COPY --from=builder /app/app_bin .

ENV PAYMENT_PROCESSOR_DEFAULT=http://localhost:8001
ENV PAYMENT_PROCESSOR_FALLBACK=http://localhost:8002

CMD ["./app_bin"]
