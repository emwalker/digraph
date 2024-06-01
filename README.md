# Digraph

Digraph is a side project and a prototype of what a next iteration on search engines might look like.  Search results are derived from manually curated topics.

* [New website](https://next.digraph.app)
* [Old website](https://digraph.app)

Changes are tracked in a history that will eventually allow traveling back in time to see what things looked like at an earlier point.  Links and topics are stored in repos, some public and some personal, which are combined to present a unified view.  More on the vision [here](https://blog.digraph.app/2022-05-29-next-steps.html).

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
