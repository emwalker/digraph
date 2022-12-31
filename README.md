# Digraph

Keep public and private repos of large numbers of links and topics that can be overlayed on top of one another in a unified view.  Every modification is tracked in a history of changes that will eventually provide the basis for traveling back in time.

* [Public site](https://digraph.app)
* [Roadmap](https://github.com/users/emwalker/projects/1/views/1)
* [Vision](https://blog.digraph.app/2022-05-29-next-steps.html)

## Screenshots

![Landing page](https://user-images.githubusercontent.com/760949/210140290-fc3fe6de-b309-4cb0-afe5-5cabcc49fb6b.png)

![Example of a topic](https://user-images.githubusercontent.com/760949/210140356-22092211-ac18-4a97-aade-09a0f60ff021.png)

## Development

Technical details:

* GraphQL backend written in Rust
* Graphs of links and topics stored in individual Git repos
* Other metadata stored in Postgres
* React/Relay client
* Webpack 5
* Server-side rendering with [Razzle](https://github.com/jaredpalmer/razzle)
* [GitHub Primer](https://styleguide.github.com/primer/) CSS

Steps for getting started with development can be found [here](https://github.com/emwalker/digraph/wiki/Getting-started-with-development).
