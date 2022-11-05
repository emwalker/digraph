# Digraph

Keep public and private repos of large numbers of links and topics that can be overlayed on top of one another in a unified view.  Every modification is tracked in a history of changes that will eventually provide the basis for traveling back in time.

* [Public site](https://digraph.app)
* [Roadmap](https://github.com/users/emwalker/projects/1/views/1)
* [Vision](https://blog.digraph.app/2022-05-29-next-steps.html)

## Screenshots

![Landing page](https://user-images.githubusercontent.com/760949/87226926-59d4aa00-c354-11ea-9082-689e079b7100.png)

![Example of a topic](https://user-images.githubusercontent.com/760949/87226972-c5b71280-c354-11ea-9305-54ee1b24068f.png)

![Synonyms for a topic](https://user-images.githubusercontent.com/760949/87226975-c9e33000-c354-11ea-83b2-ef919c570035.png)

![Links to review](https://user-images.githubusercontent.com/760949/87226978-cea7e400-c354-11ea-848e-e8462d51e908.png)

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
