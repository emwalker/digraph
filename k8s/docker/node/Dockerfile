FROM mhart/alpine-node:14

WORKDIR /app

COPY ./javascript/package.json ./
COPY ./javascript/yarn.lock ./
RUN yarn install --production


FROM mhart/alpine-node:slim-14

WORKDIR /app

COPY --from=0 /app .
ADD ./javascript/build/ /app/build/

EXPOSE 3001

CMD ["node", "build/server.js"]
