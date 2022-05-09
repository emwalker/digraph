.PHONY:

build: .PHONY
	yarn relay || yarn relay
	yarn build

check: reinstall-javacript format test build

format:
	yarn eslint
	yarn tsc

relay:
	yarn relay

reinstall-javacript:
	rm -rf node_modules
	yarn install
	rm -rf src/__generated__
	yarn relay

start:
	yarn start:prod

start-dev:
	yarn start

test:
	yarn jest
