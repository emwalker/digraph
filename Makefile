BIN              = $(GOPATH)/bin
ON               = $(BIN)/on
NODE_BIN         = $(shell npm bin)
PID              = .pid
BUNDLE           = src/static/build/bundle.js
APP              = $(shell find src -type f)
IMPORT_PATH      = $(shell pwd | sed "s|^$(GOPATH)/src/||g")
APP_NAME         = $(shell pwd | sed 's:.*/::')
GIT_HASH         = $(shell git rev-parse HEAD)
LDFLAGS          = -w -X main.commitHash=$(GIT_HASH)
GLIDE            := $(shell command -v glide 2> /dev/null)
TIMESTAMP        = $(shell date -u +%s)
LINT_DIRECTORIES = $(shell find cmd -type d ! -name "loaders" ! -name "server")
DBNAME           := $(if $(DBNAME),$(DBNAME),digraph_dev)

build: build-container-api build-container-node

build-client:
	$(MAKE) -C javascript build

build-container-api:
	docker compose build api
	docker tag emwalker/digraph-api:latest emwalker/digraph-api:$(shell cat k8s/release)

build-container-node: build-client
	docker compose build node
	docker tag emwalker/digraph-node:latest emwalker/digraph-node:$(shell cat k8s/release)

clean:
	$(MAKE) -C javascript clean
	$(MAKE) -C rust clean

check-javascript: check-rust
	$(MAKE) -C javascript check

check-git-clean:
	test -z "$(shell git diff-index --name-only HEAD --)"

check-pre-push:
	$(MAKE) -C rust check-pre-push
	$(MAKE) -C javascript check-pre-push
	test -z "$(shell git status --porcelain)"

check-rust:
	$(MAKE) -C rust check

deploy-k8s:
	kubectl apply -k k8s/overlays/prod

dump:
	pg_dump -d $(DBNAME) > data/digraph.sql

fixtures: data/fixtures.sql
	bash ./scripts/make-fixtures

logs-next:
	OVERMIND_SOCKET=./.overmind-logs.sock overmind start -f Procfile.logs-next

logs-prod:
	OVERMIND_SOCKET=./.overmind-logs.sock overmind start -f Procfile.logs-prod

migrate:
	$(MAKE) -C rust full-migration

proxy-next:
	overmind s -f Procfile.proxies-next

proxy-prod:
	overmind s -f Procfile.proxies-prod

push-docker:
	docker push emwalker/digraph-api:$(shell cat k8s/release)
	docker push emwalker/digraph-node:$(shell cat k8s/release)

push-deploy: check-git-clean build push-docker push-git deploy-k8s

push-git:
	git push origin main

reset-db:
	bash ./scripts/load-production-db
	bash ./scripts/make-fixtures
	bash ./scripts/promote-fixtures
	$(MAKE) -C rust migrate

reset-data-dir:
	rm -rf ~/data/digraph-data
	mkdir -p ~/data/digraph-data
	$(MAKE) -C rust export
	ls -l ~/data/digraph-data

save-production:
	bash ./scripts/save-production-db

start:
	overmind start -f Procfile

start-prod:
	overmind start -f Procfile.prod

start-dev:
	overmind start -f Procfile.dev

test-rust:
	$(MAKE) -C rust test

test-js:
	$(MAKE) -C javascript test

test: test-js test-rust
