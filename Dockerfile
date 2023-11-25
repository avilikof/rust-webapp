# Leveraging the pre-built Docker images with
# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.73.0 AS chef
WORKDIR /app

RUN apt-get update -y
RUN apt-get install -y libsasl2-dev
RUN apt-get install -y openssl


FROM chef AS planner
COPY . .

RUN apt-get update -y
RUN apt-get install -y libsasl2-dev
RUN apt-get install -y openssl

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json

RUN apt-get update -y
RUN apt-get install -y libsasl2-dev
RUN apt-get install -y openssl


COPY . .
RUN cargo build

FROM rust:1.73-slim AS template-rust

RUN apt-get update -y
RUN apt-get install -y libsasl2-dev
RUN apt-get install -y openssl

COPY --from=builder /app/target/debug/rust-webapp /usr/local/bin
ENTRYPOINT ["/usr/local/bin/rust-webapp"]
