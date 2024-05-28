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

prod-check: check-backend
	$(MAKE) -C client check
	$(MAKE) -C next check

check-git-clean:
	test -z "$(shell git diff-index --name-only HEAD --)"

check-pre-push:
	$(MAKE) -C backend check-pre-push
	$(MAKE) -C backend check-pre-push
	$(MAKE) -C next check-pre-push
	test -z "$(shell git status --porcelain)"

dev:
	overmind start -f Procfile.dev

dump:
	pg_dump -d $(DBNAME) > data/digraph.sql

fixtures: data/fixtures.sql
	bash ./scripts/make-fixtures

migrate:
	$(MAKE) -C backend full-migration

push-docker:
	docker push emwalker/digraph-api:$(shell cat k8s/prod-release)
	docker push emwalker/digraph-node:$(shell cat k8s/prod-release)
	docker push emwalker/digraph-node:$(shell cat k8s/next-release)

proxy:
	overmind s -f Procfile.proxies-prod

push-git:
	git push origin main

next-build-client:
	$(MAKE) -C next prod-build

next-build-container-client: prod-build-client
	docker compose build next-client
	docker tag emwalker/digraph-node:next emwalker/digraph-node:$(shell cat k8s/next-release)

prod:
	overmind start -f Procfile.prod

prod-build: prod-build-client next-build-client prod-build-api

prod-check-backend:
	$(MAKE) -C backend check

prod-deploy:
	kubectl apply -k k8s/overlays/prod

prod-logs:
	OVERMIND_SOCKET=./.overmind-logs.sock overmind start -f Procfile.prod-logs

prod-push-deploy: check-git-clean check-pre-push prod-build-containers push-docker push-git prod-deploy

prod-build-api:
	$(MAKE) -C backend build

prod-build-containers: prod-build-container-api prod-build-container-client next-build-container-client

prod-build-client:
	$(MAKE) -C client build

prod-build-container-api:
	docker compose build prod-api
	docker tag emwalker/digraph-api:latest emwalker/digraph-api:$(shell cat k8s/prod-release)

prod-build-container-client: prod-build-client
	docker compose build prod-client
	docker tag emwalker/digraph-node:latest emwalker/digraph-node:$(shell cat k8s/prod-release)

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

test-backend:
	$(MAKE) -C backend test

test-js:
	$(MAKE) -C client test

test: test-js test-backend

watch:
	$(MAKE) -C client watch
