.PHONY:

build: .PHONY
	yarn relay || yarn relay
	NODE_OPTIONS=--no-experimental-fetch yarn build

check: reinstall-javacript format test build

check-pre-push: format test

clean:
	rm -rf node_modules
	rm -rf cache
	rm -rf build

format:
	yarn eslint
	yarn tsc

relay:
	yarn relay

reinstall-javacript: clean
	yarn install
	rm -rf src/__generated__
	yarn relay

start:
	yarn start:prod

start-dev:
	NODE_OPTIONS=--no-experimental-fetch yarn start

test:
	yarn jest
