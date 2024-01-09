FROM rust:latest as builder

WORKDIR /usr/src/app

RUN apt update && apt install -y protobuf-compiler

COPY ./proto /usr/src/proto

COPY ./transaction .

RUN rustup target add aarch64-unknown-linux-musl

RUN cargo build --target aarch64-unknown-linux-musl --release

FROM alpine:latest as production

RUN apk add --no-cache protoc

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/transaction /usr/local/bin/transaction

CMD ["transaction"]

