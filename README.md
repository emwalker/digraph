# Digraph

Digraph is an app that helps keep track of large numbers of links by organizing them into topics.  No topic is too specific, and if a link points to something that you might want to refer to later on for some reason, it can be added.  Links can be found by searching within one or more subtopics.

* [Public site](https://digraph.app)
* [Long-term vision](https://blog.digraph.app/2022-05-29-next-steps.html)

## Screenshots

![Landing page](https://user-images.githubusercontent.com/760949/87226926-59d4aa00-c354-11ea-9082-689e079b7100.png)

![Example of a topic](https://user-images.githubusercontent.com/760949/87226972-c5b71280-c354-11ea-9305-54ee1b24068f.png)

![Synonyms for a topic](https://user-images.githubusercontent.com/760949/87226975-c9e33000-c354-11ea-83b2-ef919c570035.png)

![Links to review](https://user-images.githubusercontent.com/760949/87226978-cea7e400-c354-11ea-848e-e8462d51e908.png)

## Development

Technical details:

* GraphQL backend written in Rust
* Postgres
* React/Relay client
* Webpack 5
* Server-side rendering with [Razzle](https://github.com/jaredpalmer/razzle)
* [GitHub Primer](https://styleguide.github.com/primer/) CSS

Steps for getting started with development can be found [here](https://github.com/emwalker/digraph/wiki/Getting-started-with-development).
