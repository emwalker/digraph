.PHONY:

build:
	GOOS=linux GARCH=amd64 CGO_ENABLED=0 go install ./...
	cp $(shell go env GOPATH)/bin/frontend ../tmp/stage/
	cp $(shell go env GOPATH)/bin/cron ../tmp/stage/

format:
	go fmt ./...

generate:
	sqlboiler psql --output cmd/frontend/models --config ./sqlboiler.yaml --no-hooks
	go run github.com/99designs/gqlgen -config gqlgen.yml
	go generate ./...

test-integration: .PHONY
	go test -p 1 -count=1 ./test/integration/...

test: .PHONY
	go test ./cmd/frontend/... -count=1
