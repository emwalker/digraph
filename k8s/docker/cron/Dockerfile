FROM alpine:latest

RUN apk add --update ca-certificates

ADD public/ /opt/public
ADD rust/target/debug/cron /opt/cron
RUN chmod +x /opt/cron

WORKDIR /opt

CMD ["./cron"]
