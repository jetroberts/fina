FROM rust:latest as builder

WORKDIR /usr/src/app

RUN apt update && apt install -y protobuf-compiler

COPY ./proto /usr/src/proto

COPY ./transaction .

RUN cargo build --release

FROM alpine:latest as production

RUN apk add --no-cache protoc

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/transaction .

CMD ["./transaction"]

