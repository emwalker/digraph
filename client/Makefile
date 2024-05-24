build:
	rm -rf .next/standalone
	yarn build
	cp -r .next/static .next/standalone/.next
	cp -r public .next/standalone/.next

check:
	yarn jest
	yarn lint
	yarn build

dev:
	yarn dev

prod:
	PORT=3000 HOSTNAME="0.0.0.0" node .next/standalone/server.js
