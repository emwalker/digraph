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

relay:
	yarn relay
	yarn tsc

reinstall-javacript: clean
	yarn install
	rm -rf src/__generated__
	mkdir src/__generated__
	yarn relay

dev:
	NODE_OPTIONS=--no-experimental-fetch yarn start

prod:
	yarn start:prod

test:
	yarn jest
