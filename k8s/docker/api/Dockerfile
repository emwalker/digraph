FROM rust:1.66-buster as builder
WORKDIR /usr/src/digraph
COPY ./rust .
RUN cargo build --release --bin api

FROM debian:buster-slim
LABEL Name=api
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/digraph/target/release/api /app/api
CMD ["/app/api"]
