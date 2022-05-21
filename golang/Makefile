.PHONY:

build:
	GOOS=linux GARCH=amd64 CGO_ENABLED=0 go install ./...
	cp $(shell go env GOPATH)/bin/frontend ../tmp/stage/
	cp $(shell go env GOPATH)/bin/cron ../tmp/stage/

format:
	go fmt ./...

check: test lint
	go mod tidy
	test -z "$(shell git diff-index --name-only HEAD --)"
	go vet ./...

generate:
	sqlboiler psql --output internal/models --config ./sqlboiler.yaml --no-hooks
	go run github.com/99designs/gqlgen -config gqlgen.yml
	go generate ./...

lint:
	staticcheck ./...

start:
	PORT=8080 go run cmd/frontend/frontend.go --log 1

test-integration: .PHONY
	go test -p 1 -count=1 ./test/integration/...

test: .PHONY
	go test ./internal/... -count=1
