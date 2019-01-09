BIN           = $(GOPATH)/bin
ON            = $(BIN)/on
NODE_BIN      = $(shell npm bin)
PID           = .pid
BUNDLE        = src/static/build/bundle.js
APP           = $(shell find src -type f)
IMPORT_PATH   = $(shell pwd | sed "s|^$(GOPATH)/src/||g")
APP_NAME      = $(shell pwd | sed 's:.*/::')
GIT_HASH      = $(shell git rev-parse HEAD)
LDFLAGS       = -w -X main.commitHash=$(GIT_HASH)
GLIDE         := $(shell command -v glide 2> /dev/null)
TIMESTAMP     = $(shell date -u +%s)
DBNAME        = digraph_dev

kill:
	@killall server 2>/dev/null || true
	@killall node 2>/dev/null || true

start: kill
	@yarn relay --watch &
	@yarn start &
	@go run server.go --dev --log 1

start-debug: kill
	@yarn relay --watch &
	@yarn start &
	@go run server.go --dev --log 2

lint:
	golint models resolvers server services
	yarn eslint

install:
	@yarn install
	@dep ensure

migrate-up:
	$(foreach database,$(DBNAME),\
		migrate -database "postgres://postgres@localhost:5432/$(database)?sslmode=disable" \
			-source file://migrations up 1 ;\
	)

migrate-down:
	$(foreach database,$(DBNAME),\
		migrate -database "postgres://postgres@localhost:5432/$(database)?sslmode=disable" \
			-source file://migrations down 1 ;\
	)

.PHONY:

test-integration:
	go test ./test/integration/...

test: .PHONY
	go test ./models ./resolvers ./server ./services
	yarn jest

format:
	yarn flow
	go fmt ./...
	git diff-index --quiet HEAD --

check: lint format test test-integration

load:
	dropdb $(DBNAME)
	createdb $(DBNAME)
	psql -d $(DBNAME) --set ON_ERROR_STOP=on < data/fixtures.sql

dump:
	pg_dump -d $(DBNAME) > data/digraph.sql

fixtures: dump
	cp data/digraph.sql data/fixtures.sql

clean:
	rm -f public/webpack/*.js* public/webpack/*.css*

build-client: clean
	yarn build

build-frontend:
	GOOS=linux GARCH=amd64 CGO_ENABLED=0 go install -v -a
	mkdir -p tmp/stage
	cp $(shell go env GOPATH)/bin/linux_amd64/digraph tmp/stage/
	docker build . -t digraph:latest -f k8s/docker/Dockerfile
	docker tag digraph:latest emwalker/digraph:$(shell cat k8s/release)

build: build-client build-frontend

up:
	docker run -p 5432:5432 -p 8080:8080 --env-file tmp/env.list --rm -it digraph

push:
	docker push emwalker/digraph:$(shell cat k8s/release)

deploy:
	kubectl config use-context do-default
	kubectl apply -f k8s/cluster

proxy:
	kubectl port-forward --namespace default svc/postgres-postgresql 5433:5432
