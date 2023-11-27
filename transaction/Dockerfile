FROM rust:latest as builder

WORKDIR /usr/src/app

RUN apt update && apt install -y protobuf-compiler

COPY ../proto /usr/src/proto

COPY . .

RUN cargo build --release

FROM alpine:latest as production

ARG PACKAGE_NAME

RUN if [ "$PACKAGE_NAME" = "" ]; then \
    echo "PACKAGE_NAME is not set"; \
    exit 1; \
    fi

RUN apk add --no-cache protoc

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/${PACKAGE_NAME} .

CMD ["./$PACKAGE_NAME"]

