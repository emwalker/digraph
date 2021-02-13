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

# Might core dump on Linux
kill:
	@killall server 2>/dev/null || true
	@killall node 2>/dev/null || true
	@pkill -ABRT -f node || true
	@pkill -TERM -f frontend || true

start:
	@yarn relay --watch &
	#@redis-server /usr/local/etc/redis.conf &
	@go run cmd/frontend/frontend.go --dev --log 1 &
	@yarn start

start-prod: kill
	@redis-server /usr/local/etc/redis.conf &
	@go run cmd/frontend/frontend.go --dev --log 1 &
	@yarn start:prod

start-debug: kill
	@yarn relay --watch &
	@redis-server /usr/local/etc/redis.conf &
	@go run cmd/frontend/frontend.go --dev --log 2 &
	@yarn start

lint-js:
	yarn eslint

lint: lint-js
	golint -set_exit_status $(LINT_DIRECTORIES)

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

generate:
	sqlboiler psql --output cmd/frontend/models --config ./sqlboiler.yaml --no-hooks
	go generate ./...

.PHONY:

test-integration: .PHONY
	go test -p 1 -count=1 ./test/integration/...

test-js: .PHONY
	yarn jest

test-go: .PHONY
	go test ./cmd/frontend/...

test: test-js test-go

format-js: lint-js
	yarn flow

format: lint format-js
	go fmt ./...
	git diff --quiet

check-js: format-js test-js
	yarn relay
	yarn outdated

check: generate format test test-integration

dump:
	pg_dump -d $(DBNAME) > data/digraph.sql

fixtures: data/fixtures.sql
	bash ./scripts/make-fixtures

load-fixtures:
	bash ./scripts/load-fixtures
	psql $(DBNAME) < queries/transitive-closure.sql

load-production:
	bash ./scripts/load-production-db
	psql $(DBNAME) < queries/transitive-closure.sql

recreate-transitive-closures:
	psql $(DBNAME) < queries/clear-transitive-closure.sql
	psql $(DBNAME) < queries/transitive-closure.sql

save-production:
	bash ./scripts/save-production-db

build-client:
	yarn relay
	yarn build

build-executables:
	GOOS=linux GARCH=amd64 CGO_ENABLED=0 go install ./...
	mkdir -p tmp/stage
	cp $(shell go env GOPATH)/bin/linux_amd64/frontend tmp/stage/
	cp $(shell go env GOPATH)/bin/linux_amd64/cron tmp/stage/

build-container-api:
	docker-compose build api
	docker tag emwalker/digraph-api:latest emwalker/digraph-api:$(shell cat k8s/release)

build-container-node: build-client
	docker-compose build node
	docker tag emwalker/digraph-node:latest emwalker/digraph-node:$(shell cat k8s/release)

build-container-cron:
	docker-compose build cron
	docker tag emwalker/digraph-cron:latest emwalker/digraph-cron:$(shell cat k8s/release)

build: build-executables build-container-cron build-container-api build-container-node

push:
	docker push emwalker/digraph-cron:$(shell cat k8s/release)
	docker push emwalker/digraph-api:$(shell cat k8s/release)
	docker push emwalker/digraph-node:$(shell cat k8s/release)

deploy:
	kubectl config use-context do-sfo2-do-cluster
	kubectl apply -f k8s/cluster

proxy:
	kubectl port-forward --namespace default svc/postgres-postgresql 5433:5432
