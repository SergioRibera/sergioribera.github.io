# syntax=docker/dockerfile:1
FROM rust:1.68.2-alpine as base

WORKDIR /pro/building

RUN apk add --no-cache musl-dev=1.2.3-r4 libsass=3.6.5-r0 npm=9.1.2-r0 && \
    npm i -g sass && \
    cargo install trunk cargo-make --locked && \
    rustup target add wasm32-unknown-unknown && \
    mkdir /pro/models

COPY ./models /pro/models
COPY ./web /pro/building
COPY ./rust-toolchain.toml /pro/building

RUN cargo make release-build-web && \
    cp see.conf dist/

#
# Server Launcher
#
FROM wyhaya/see:latest

WORKDIR /app

COPY --from=base /pro/building/dist .

CMD ["/see", "-c", "see.conf"]
