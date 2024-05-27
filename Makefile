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

build-api:
	$(MAKE) -C backend build

build-containers: build-container-api build-container-node

build-client:
	$(MAKE) -C client build

build-container-api:
	docker compose build api
	docker tag emwalker/digraph-api:latest emwalker/digraph-api:$(shell cat k8s/release)

build-container-node: build-client
	docker compose build client
	docker tag emwalker/digraph-node:latest emwalker/digraph-node:$(shell cat k8s/release)

build-prod: build-client build-api

clean:
	$(MAKE) -C client clean
	$(MAKE) -C backend clean

check-client: check-backend
	$(MAKE) -C client check

check-git-clean:
	test -z "$(shell git diff-index --name-only HEAD --)"

check-pre-push:
	$(MAKE) -C backend check-pre-push
	$(MAKE) -C client check-pre-push
	test -z "$(shell git status --porcelain)"

check-backend:
	$(MAKE) -C backend check

deploy-k8s:
	kubectl apply -k k8s/overlays/prod

dev:
	overmind start -f Procfile.dev

dump:
	pg_dump -d $(DBNAME) > data/digraph.sql

fixtures: data/fixtures.sql
	bash ./scripts/make-fixtures

logs-next:
	OVERMIND_SOCKET=./.overmind-logs.sock overmind start -f Procfile.logs-next

logs-prod:
	OVERMIND_SOCKET=./.overmind-logs.sock overmind start -f Procfile.logs-prod

migrate:
	$(MAKE) -C backend full-migration

proxy-next:
	overmind s -f Procfile.proxies-next

proxy-prod:
	overmind s -f Procfile.proxies-prod

push-docker:
	docker push emwalker/digraph-api:$(shell cat k8s/release)
	docker push emwalker/digraph-node:$(shell cat k8s/release)

push-deploy: check-git-clean build-containers push-docker push-git deploy-k8s

push-git:
	git push origin main

reset-db:
	bash ./scripts/load-production-db
	bash ./scripts/make-fixtures
	bash ./scripts/promote-fixtures
	$(MAKE) -C backend migrate

reset-data-dir:
	rm -rf ~/data/digraph-data
	mkdir -p ~/data/digraph-data
	$(MAKE) -C backend export
	ls -l ~/data/digraph-data

save-production:
	bash ./scripts/save-production-db

prod:
	overmind start -f Procfile.prod

test-backend:
	$(MAKE) -C backend test

test-js:
	$(MAKE) -C client test

test: test-js test-backend

watch:
	$(MAKE) -C client watch
