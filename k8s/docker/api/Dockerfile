FROM rust:1-alpine as builder
WORKDIR /usr/src/digraph
COPY ./rust .
RUN apk add --no-cache musl-dev
RUN apk add --update pkgconfig openssl-dev
RUN cargo build --release --bin api

FROM alpine
LABEL Name=frontend Version=0.1.0
RUN apk add --update ca-certificates openssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/digraph/target/release/api /app/api
CMD ["/app/api"]
