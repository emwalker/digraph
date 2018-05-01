BIN           = $(GOPATH)/bin
ON            = $(BIN)/on
NODE_BIN      = $(shell npm bin)
PID           = .pid
GO_FILES      = $(shell find ./server -type f -name "*.go")
BUNDLE        = client/static/build/bundle.js
APP           = $(shell find client -type f)
IMPORT_PATH   = $(shell pwd | sed "s|^$(GOPATH)/src/||g")
APP_NAME      = $(shell pwd | sed 's:.*/::')
TARGET        = $(BIN)/$(APP_NAME)
GIT_HASH      = $(shell git rev-parse HEAD)
LDFLAGS       = -w -X main.commitHash=$(GIT_HASH)
GLIDE         := $(shell command -v glide 2> /dev/null)
TIMESTAMP     = $(shell date -u +%s)
DATABASES     = digraffe_dev digraffe_test

build: $(ON) clean $(TARGET)

clean:
	@rm -rf client/static/build/*

$(ON):
	go install $(IMPORT_PATH)/vendor/github.com/olebedev/on

$(TARGET):
	@go build -ldflags '$(LDFLAGS)' -o $@ $(IMPORT_PATH)/server

kill:
	@kill `cat $(PID)` || true

serve: $(ON) clean $(TARGET) restart
	@yarn relay --watch &
	@yarn start &
	@$(ON) -m 2 $(GO_FILES) | xargs -n1 -I{} make restart || make kill

restart: LDFLAGS += -X main.debug=true
restart: kill $(TARGET)
	@echo restart the app...
	@$(TARGET) run & echo $$! > $(PID)

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
	@cayley load --config=./cayley.cfg.json --load=./data/digraffe.nq

dump:
	@cayley dump --config=./cayley.cfg.json --dump=./data/digraffe.nq
