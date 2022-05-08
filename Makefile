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

build: build-executables build-container-cron build-container-api build-container-node

build-client:
	$(MAKE) -C javascript build

build-container-api:
	docker-compose build api
	docker tag emwalker/digraph-api:latest emwalker/digraph-api:$(shell cat k8s/release)

build-container-cron:
	docker-compose build cron
	docker tag emwalker/digraph-cron:latest emwalker/digraph-cron:$(shell cat k8s/release)

build-container-node: build-client
	docker-compose build node
	docker tag emwalker/digraph-node:latest emwalker/digraph-node:$(shell cat k8s/release)

build-executables:
	$(MAKE) -C golang build

deploy:
	kubectl config use-context digraph-production
	kubectl apply -f k8s/cluster

dump:
	pg_dump -d $(DBNAME) > data/digraph.sql

fixtures: data/fixtures.sql
	bash ./scripts/make-fixtures

generate:
	$(MAKE) -C golang $@

load-fixtures:
	bash ./scripts/load-fixtures
	psql $(DBNAME) < queries/transitive-closure.sql

load-production:
	bash ./scripts/load-production-db
	psql $(DBNAME) < queries/transitive-closure.sql

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

proxy:
	kubectl port-forward --namespace default svc/postgres-postgresql 5433:5432

push:
	docker push emwalker/digraph-cron:$(shell cat k8s/release)
	docker push emwalker/digraph-api:$(shell cat k8s/release)
	docker push emwalker/digraph-node:$(shell cat k8s/release)

recreate-transitive-closures:
	psql $(DBNAME) < queries/clear-transitive-closure.sql
	psql $(DBNAME) < queries/transitive-closure.sql

save-production:
	bash ./scripts/save-production-db

test-go:
	$(MAKE) -C golang test

test-js:
	$(MAKE) -C javascript test

test: test-js test-go
