# Digraph

Digraph is a web app that helps to keep track of large numbers of links by organizing them into topics.  The aim is to make it straightforward to get back to anything you've read or wanted to read later on.

* [Website](https://digraph.app)
* [Blog](https://blog.digraph.app/)

Changes are tracked in a history that will eventually allow traveling back in time to see what things looked like at an earlier point.  Links and topics are stored in repos, some public and some personal, that are combined into a single view.  More on the vision [here](https://blog.digraph.app/2022-05-29-next-steps.html).

This is a personal project and is a work in progress.

## Screenshots

![Landing page](https://user-images.githubusercontent.com/760949/210140290-fc3fe6de-b309-4cb0-afe5-5cabcc49fb6b.png)

![Example of a topic](https://user-images.githubusercontent.com/760949/210140356-22092211-ac18-4a97-aade-09a0f60ff021.png)

## Development

Technical details:

* GraphQL backend written in Rust
* Graphs of links and topics stored in individual Git repos
* Postgres
* Redis
* React/Relay client
* Webpack 5
* Server-side rendering with [Razzle](https://github.com/jaredpalmer/razzle)
* [GitHub Primer](https://styleguide.github.com/primer/) CSS
