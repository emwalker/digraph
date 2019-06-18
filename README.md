# Digraph

Keep track of links in a mind-map like network of topics.

* Public site [here](https://digraph.app)
* Recent updates are covered [here](https://blog.digraph.app)
* Some of the use cases that are contemplated are described [here](https://github.com/emwalker/digraph/wiki)

## Technical details

* GraphQL backend written in Golang
* Postgres
* React/Relay client
* Webpack 4
* [volatiletech/sqlboiler](https://github.com/volatiletech/sqlboiler)
* [99designs/gqlgen](https://github.com/99designs/gqlgen)
* [GitHub Primer](https://styleguide.github.com/primer/) CSS

## Getting started

Requirements

* Postgres 10
* Redis 5
* Go 1.11
* yarn

Set up the project:
```
$ mkdir $GOPATH/src/github.com/emwalker
$ cd $GOPATH/src/github.com/emwalker
$ git clone git@github.com:emwalker/digraph.git
$ cd digraph
$ make load-fixtures
$ go get -u github.com/99designs/gqlgen
$ make generate
$ go get -u ./...
$ yarn install
$ make test
$ make test-integration
```

Set up the login session:
```
$ make build-client
$ redis-server /usr/local/etc/redis.conf # In one terminal
$ go run cmd/frontend/frontend.go -dev # In another terminal
# Go to localhost:8080 in a browser and sign in with your Github account, possibly *twice*, if the first time
# doesn't work. Now you can CTRL-C to quit both go and redis
```

Run the app in development:
```
$ make start
```

## Screenshot

![Screenshot](https://user-images.githubusercontent.com/760949/58448929-aca6c000-80c6-11e9-83b2-6fa408f2cced.png)
