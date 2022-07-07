Soon:
* Update link title, don't fetch html if we already have the link
* Activity feed using anonymous ids + user lookups
* Restrict on org/repo prefix
* Add limit to topic results (why did "Reading list" take such a long time?)

Things that would be nice to eventually get to:
* Populate locale dropdown using a GraphQL field
* Update [getting-started steps on Wiki](https://github.com/emwalker/digraph/wiki/Getting-started-with-development)
* Switch the topic view to using search without descendents, and add a cursor for paging
* Add paging
* Zero-copy search results
* Move over to Primer React components
* Revisit UI look and feel
* Get query info working again (the text and topics that appear in the search bar)
* Verify the server secret before deleting a session
* Drop trigram indexes if they're not being used
* Add a resource_path column to the topics table
* Simplify the table used for recent activity
* Rename Topic.links to Topic.childLinks
* Unify the search and the topic code.  When there's paging, you should be paging through the topics and then the links on a topic page, which implies a single ordered list, with topics ordered at the start.  What we're doing right now is showing a section of topics and a section of links.
* Add in role directives to the Graphql schema.
* Revisit how the Topic.viewerCanUpdate boolean is evaluated.  Right now we're just looking at whether a repo is private or not.  We also need to take into account things like whether the repo is in an org the viewer is a member of, etc.
* Bugfix for Time / Science / Agriculture and farming test case
* Always show dates in topics with a date in alll contexts
* Live search shows synonyms
* Add an owner id to organizations
* Disambiguate topics with same names in different repos (e.g., Everything)
* Parse PDF metadata to get the title (e.g., with https://docs.rs/crate/skia-safe/latest)
