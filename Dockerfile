FROM rust:1.87-alpine AS builder

RUN apk update && apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    curl \
    openssl-libs-static \
    bash

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /app/target/release/badge_forge /app/badge_forge

RUN chmod +x /app/badge_forge

EXPOSE 4000

CMD ["/app/badge_forge"]
