FROM rust:1.72-buster as builder
WORKDIR /usr/src/digraph
COPY ./rust .
RUN cargo build --release --bin api

FROM debian:buster-slim
LABEL Name=api
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/digraph/target/release/api /app/api

ADD ./scripts/pack-data /tmp/pack-data
ADD ./scripts/container-pack-data /tmp/container-pack-data
RUN chmod +x /tmp/container-pack-data

ADD ./scripts/container-save-db-snapshot /tmp/container-save-db-snapshot
RUN chmod +x /tmp/container-save-db-snapshot

CMD ["/app/api"]
