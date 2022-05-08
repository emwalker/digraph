.PHONY:

build: .PHONY
	yarn relay || yarn relay
	yarn build

check-js: reinstall-javacript format-js test-js build-client
	yarn outdated

format-js: lint-js
	yarn tsc

format: lint format-js
	go fmt ./...
	git diff --quiet

relay:
	yarn relay

reinstall-javacript:
	rm -rf node_modules
	yarn install
	rm -rf src/__generated__
	yarn relay

start:
	@yarn relay --watch &
	@go run cmd/frontend/frontend.go --log 1 &
	@yarn start

start-prod:
	@go run cmd/frontend/frontend.go --log 1 &
	@yarn start:prod

start-debug:
	@yarn relay --watch &
	@go run cmd/frontend/frontend.go --log 2 &
	@yarn start

test:
	yarn jest

lint:
	yarn eslint
