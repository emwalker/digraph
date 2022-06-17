FROM alpine:latest

RUN apk add --update ca-certificates

ADD ./rust/target/release/frontend /opt/frontend
RUN chmod +x /opt/frontend

EXPOSE 8080

WORKDIR /opt

CMD ["./frontend"]
