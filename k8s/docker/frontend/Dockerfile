FROM rust:1.61.0-buster as builder
WORKDIR /usr/src/digraph
COPY ./rust .
RUN cargo build --release --bin frontend

FROM debian:buster-slim
LABEL Name=frontend Version=0.1.0
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/digraph/target/release/frontend /app/frontend
CMD ["/app/frontend"]
