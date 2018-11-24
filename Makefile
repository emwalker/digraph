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

build: clean

clean:
	@rm -rf src/static/build/*

kill:
	@killall server 2>/dev/null || true
	@killall node 2>/dev/null || true

start: kill clean
	@yarn relay --watch &
	@yarn start &
	@go run server.go

lint:
	golint models server resolvers
	yarn run eslint

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
	go test ./models ./resolvers
	yarn jest

format:
	yarn eslint
	yarn flow
	go fmt ./...
	git diff-index --quiet HEAD --

check: format test test-integration

load:
	dropdb $(DBNAME)
	createdb $(DBNAME)
	psql -d $(DBNAME) --set ON_ERROR_STOP=on < data/fixtures.sql

dump:
	pg_dump -d $(DBNAME) > data/digraph.sql

fixtures: dump
	cp data/digraph.sql data/fixtures.sql

build:
	yarn build

deploy: build
	gcloud app deploy
