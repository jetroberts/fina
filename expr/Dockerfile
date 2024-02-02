FROM rust:latest as builder

WORKDIR /usr/src/app

RUN rustup target add aarch64-unknown-linux-musl

RUN apt-get update && apt-get install -y musl-tools gcc-aarch64-linux-gnu

COPY ./expr .

ENV SQLX_OFFLINE=true

RUN cargo build --target aarch64-unknown-linux-musl --release

FROM alpine:latest as production

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/expr /usr/local/bin/expr

CMD ["expr", "-s"]

