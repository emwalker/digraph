# Digraph

Digraph is a web app that helps to keep track of large numbers of links by organizing them into topics.  The aim is to make it straightforward to get back to anything you've read or wanted to read later on.

* [New website](https://next.digraph.app)
* [Old website](https://digraph.app)
* [Blog](https://blog.digraph.app/)

Changes are tracked in a history that will eventually allow traveling back in time to see what things looked like at an earlier point.  Links and topics are stored in repos, some public and some personal, that are combined into a single view.  More on the vision [here](https://blog.digraph.app/2022-05-29-next-steps.html).

This is a personal project and is a work in progress.

## Screenshot

![2024-06-01_15-28](https://github.com/emwalker/digraph/assets/760949/ec021b57-7826-4723-98c0-e9a21a0a473f)

## Development

Technical details:

* Next.js, Mantine, React
* GraphQL backend written in Rust
* Object graphs stored in per-account Git repos
* Postgres
* Redis
