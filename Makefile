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

build-client:
	$(MAKE) -C javascript build

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
	#kubectl config use-context digraph-production
	kubectl apply -f k8s/cluster

dump:
	pg_dump -d $(DBNAME) > data/digraph.sql

export:
	$(MAKE) -C rust export

fixtures: data/fixtures.sql
	bash ./scripts/make-fixtures

load-fixtures:
	bash ./scripts/load-fixtures

load-production:
	bash ./scripts/load-production-db
	$(MAKE) -C rust full-migration

logs:
	OVERMIND_SOCKET=./.overmind-logs.sock overmind start -f Procfile.logs

migrate:
	$(MAKE) -C rust full-migration

proxy:
	kubectl port-forward --namespace default svc/postgres-postgresql 5431:5432

push-docker:
	docker push emwalker/digraph-cron:$(shell cat k8s/release)
	docker push emwalker/digraph-api:$(shell cat k8s/release)
	docker push emwalker/digraph-node:$(shell cat k8s/release)

push-deploy: check-git-clean build push-docker push-git deploy-k8s

push-git:
	git push origin main

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
