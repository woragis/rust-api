FROM rust:1.84.0 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo fetch

RUN cargo build --release


# Debian option
# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y libssl-dev && apt-get clean

# Ubuntu option
FROM ubuntu:latest
RUN apt-get update && apt-get install -y libssl3

# Alpine option
# FROM alpine:latest
# RUN apk add --no-cache openssl

COPY --from=builder /app/target/release/api /usr/local/bin/rust_api

ENTRYPOINT [ "rust_api" ]
