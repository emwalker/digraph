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
```

Run the tests:
```
$ make test
$ make test-integration
```

Run the app in development:
```
$ make start
```

## Screenshots

![Screenshot](https://user-images.githubusercontent.com/760949/59727248-24619980-91f2-11e9-9067-62042fbda222.png)

![Screenshot](https://user-images.githubusercontent.com/760949/59727250-275c8a00-91f2-11e9-87b8-3aed54ad0335.png)
