BIN           = $(GOPATH)/bin
ON            = $(BIN)/on
NODE_BIN      = $(shell npm bin)
PID           = .pid
GO_FILES      = $(shell find ./server -type f -name "*.go")
BUNDLE        = src/static/build/bundle.js
APP           = $(shell find src -type f)
IMPORT_PATH   = $(shell pwd | sed "s|^$(GOPATH)/src/||g")
APP_NAME      = $(shell pwd | sed 's:.*/::')
GIT_HASH      = $(shell git rev-parse HEAD)
LDFLAGS       = -w -X main.commitHash=$(GIT_HASH)
GLIDE         := $(shell command -v glide 2> /dev/null)
TIMESTAMP     = $(shell date -u +%s)
DATABASES     = digraffe_dev digraffe_test

build: clean

clean:
	@rm -rf src/static/build/*

kill:
	@killall node 2>/dev/null || true
	@killall digraffe 2>/dev/null || true

serve: clean kill
	@yarn relay --watch &
	@yarn start &
	@go run server/main.go

lint:
	@yarn run eslint || true
	@golint $(GO_FILES) || true

install:
	@yarn install
	@dep ensure

migrate-up:
	$(foreach database,$(DATABASES),\
		migrate -database "postgres://postgres@localhost:5432/$(database)?sslmode=disable" \
			-source file://migrations up 1 ;\
	)

migrate-down:
	$(foreach database,$(DATABASES),\
		migrate -database "postgres://postgres@localhost:5432/$(database)?sslmode=disable" \
			-source file://migrations down 1 ;\
	)

test:
	yarn jest
	go test ./...

format:
	yarn eslint
	yarn flow
	go fmt ./...
	git diff-index --quiet HEAD --

check: format test

load:
	@cayley load --config=./cayley.cfg.json --load=./data/personal.nq

dump:
	@cayley dump --config=./cayley.cfg.json --dump=./data/personal.nq

repl:
	@cayley repl --config=./cayley.cfg.json
