FROM node:20-alpine3.17

WORKDIR /app

COPY ./package.json ./
COPY ./yarn.lock ./
RUN yarn install --production --ignore-platform


FROM mhart/alpine-node:slim-14

WORKDIR /app

COPY --from=0 /app .
ADD ./build/ /app/build/

EXPOSE 3000

CMD ["node", "build/server.js"]
