FROM rust:alpine AS builder

WORKDIR /usr/src/rust-ip-server

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

COPY Cargo.toml Cargo.lock ./
RUN mkdir src

COPY src ./src/

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache libgcc libstdc++

COPY --from=builder /usr/src/rust-ip-server/target/release/rust-ip-server .

ENV PORT=8080

CMD ["sh", "-c", "./rust-ip-server"]
