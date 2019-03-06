FROM alpine:latest

RUN apk add --update ca-certificates

ADD public/ /opt/public
ADD tmp/stage/cron /opt/cron
RUN chmod +x /opt/cron

WORKDIR /opt

CMD ["./cron"]
