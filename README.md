# digraph
Find articles, blog posts and documents you've skimmed in the past and wanted to read later on by associating them with topics of arbitrary specificity.

![Screenshot](https://user-images.githubusercontent.com/760949/50575106-9fa1d100-0db3-11e9-9fa4-08659c83dde9.png)

Topics live within a hierarchy, and it's possible to place a subtopic under more than one parent topic. This removes the need to choose whether "Golang articles" belongs under "Golang" or "Articles about programming". Links and topics can be added to a general "wiki" repository visible to everyone or to a private repository visible only to yourself.

## Future

The current roadmap:
* Searching within a topic.
* Contexts, to keep track of things that have to do with an opinion or goal of the viewer which others might not share, such as "Links to read" or "Beautiful architecture".
* Filters, which filter what is being viewed by restricting links and topics to specific repositories and contexts.
* Make it easy for others to set up a dev environment.

## Technical details
* GraphQL backend written on Golang
* Postgres
* React/Relay client
* Webpack 4
* [volatiletech/sqlboiler](https://github.com/volatiletech/sqlboiler)
* [99designs/gqlgen](https://github.com/99designs/gqlgen)
* [GitHub Primer](https://styleguide.github.com/primer/) CSS
