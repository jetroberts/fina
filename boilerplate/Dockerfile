FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM alpine:latest as production

ARG PACKAGE_NAME

RUN if [ "$PACKAGE_NAME" = "" ]; then \
    echo "PACKAGE_NAME is not set"; \
    exit 1; \
    fi

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/${PACKAGE_NAME} .

CMD ["./$PACKAGE_NAME"]

