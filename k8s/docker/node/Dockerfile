FROM mhart/alpine-node:12

WORKDIR /app

COPY package.json ./
COPY yarn.lock ./
RUN yarn install
ADD build/ /app/build/

FROM mhart/alpine-node:slim-12
WORKDIR /app
COPY --from=0 /app .

EXPOSE 3001

CMD ["node", "build/server.js"]
