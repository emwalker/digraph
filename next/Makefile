prod-build:
	rm -rf .next/standalone
	yarn build
	cp -r .next/static .next/standalone/.next

compile:
	yarn run compile

check: check-pre-push
	yarn build

check-pre-push:
	yarn jest
	yarn lint

dev:
	yarn dev

prod:
	PORT=3002 HOSTNAME="0.0.0.0" node .next/standalone/server.js

test:
	yarn jest

watch: compile
	yarn run compile -w
