Before switchover:
* Ensure that organizations, users, and repos are being deleted when a person asks for their account to be destroyed
* Fill in implementation for link and topic counts on home page, possibly using redis, possibly adding up across visible repos, possibly using grep
  + grep -lr -E "^kind: Topic" objects/ | wc -l
  + grep -lr -E "^kind: Link" objects/ | wc -l
* Update db/schema.sql and add a make task to load it
* Merge the contents of items with the same id and different repo prefixes
* Decide whether to support link reviews or remove the feature
* Git: Start work on git rebases and commits
* Git: Sort out logistics of merging branch and deploying
* Git: Figure out how to upload the repos
* Git: Figure out how to download a repo
* Cutover: run some experiments using minikube
* Cutover: Current snapshot of database
* Cutover: Export people's repos
* Cutover: Run addititive migrations against database
* Cutover: Make the switchover live
* Cutover: Run migrations to clean up tables that are no longer needed

Later:
* Use a queue for rebasing commits before mering, instead of retrying
* Look into using iterators for the search code instead of copying large buffers around
* Keep tabs on whether the ordering of changes is stable
* Get the topic upsert working when the topic already exists
* Show search input in input box when loaded from a url
* Look into a timestamp format that is suitable for older dates (e.g., 500 BC) and possibly geological timescales
* Figure out how to display the same change in different contexts -- e.g., a topic is deleted, and its contents merged into the parent topics.  What does the change look like in each change history?
* Figure out how to make history entries outlive the topic and links when they're deleted.  I think this means denormalizing title, url and synonym info in the changes themeslves, mabye with one level of indirection? Or maybe it means putting a marker on the deleted item to indicate that it's deleted rather than actually deleting it.  Maybe change "delete" to "remove" in the UI if this approach ends up being used.  There's also the problem of historical names and titles that change.
* Figure out how to display the matching synonym in search results (e.g., Radiocarbon dating v. Carbon-14 dating)
* Update link title, don't fetch html if we already have the link
* Populate locale dropdown using a GraphQL field
* Update [getting-started steps on Wiki](https://github.com/emwalker/digraph/wiki/Getting-started-with-development)
* Switch the topic view to using search without descendents, and add a cursor for paging
* Show synonyms rather than names in search results
* Add paging
* Use a service account for creating a new repo on session upsert
* Zero-copy search results
* Make /wiki/ the default prefix when someone logs in for the first time
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
