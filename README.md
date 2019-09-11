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
* Server-side rendering with [Razzle](https://github.com/jaredpalmer/razzle)
* [volatiletech/sqlboiler](https://github.com/volatiletech/sqlboiler)
* [99designs/gqlgen](https://github.com/99designs/gqlgen)
* [GitHub Primer](https://styleguide.github.com/primer/) CSS

## Getting started

Requirements

* Postgres 10
* Redis 5
* Go 1.11
* Yarn

Set up the project:
```
$ export DIGRAPH_POSTGRES_CONNECTION="dbname=digraph_dev user=postgres sslmode=disable"
$ mkdir $GOPATH/src/github.com/emwalker
$ cd $GOPATH/src/github.com/emwalker
$ git clone git@github.com:emwalker/digraph.git
$ cd digraph
$ make load-fixtures
$ go get -u github.com/golang-migrate/migrate
$ go get -u github.com/99designs/gqlgen
$ make generate
$ go get -u ./...
$ yarn install
$ yarn relay
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

In order to log in, you'll need to configure a dedicated OAuth app in GitHub and set the following environment
variables:
```
$ export DIGRAPH_GITHUB_CLIENT_ID="..."
$ export DIGRAPH_GITHUB_CLIENT_SECRET="..."
```

## Screenshots

![Screenshot](https://user-images.githubusercontent.com/760949/64667069-b81fbc00-d415-11e9-9919-0be315eecb67.png)

![Screenshot](https://user-images.githubusercontent.com/760949/59727248-24619980-91f2-11e9-9067-62042fbda222.png)

![Screenshot](https://user-images.githubusercontent.com/760949/59727250-275c8a00-91f2-11e9-87b8-3aed54ad0335.png)
