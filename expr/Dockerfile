FROM rust:latest as builder

WORKDIR /usr/src/app

RUN rustup target add x86_64-unknown-linux-musl

COPY ./expr .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest as production

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/expr /usr/local/bin/expr

CMD ["expr", "-s"]

