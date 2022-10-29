Next:

Before switchover:
* Bug: when the private repo is selected, toggling the timerange prefix doesn't work
* Bug: client: some history entries are not displaying properly
* Bug: when changing from private repo to Wiki, the edit form doesn't appear for a repo topic
* Bug: a reference to XupOirPbeLkV5aJfU-TsqPxQ5gNf33Lq-40g_3Pun8Q (Reading list) is improperly being added to the Wiki repo
* Get alerts working again (test case: you add a synonym that already exists)
* Add support for the "Ask if what to do" case for topics (test case: add a topic that already exists in the repo)
* Show a blank edit form when a repo is selected and there's no reference for the topic/link
* When upserting a topic that exists in a private repo into the Wiki repo, get confirmation first
* When upserting a topic that exists in a private repo into the Wiki repo, add a non-nil details struct
* Verify that history entries are working for cross-repo mutations
* Verify that account creation still works
* Verify that account deletion still works
* Verify that topics still show up at the top of their own topic search
* Verify that searches can be scoped to the current topic instead of "Everything"
* Verify that things work when not logged in
* Clean up database: odds and ends
* Git: Start work on git rebases and commits
* Git: Sort out logistics of merging branch and deploying
* Git: provide UX to download personal repo
* Finish reading the series on Git internals (https://github.blog/2022-08-29-gits-database-internals-i-packed-object-store/)
* Cutover: Figure out how to get an RWX pvc working on Linode (use Ceph?)
* Cutover: Take snapshot of database
* Cutover: Export people's repos
* Cutover: Run addititive migrations against database
* Cutover: Make the switchover live
* Cutover: Run migrations to clean up tables that are no longer needed


Later:
* Repo topics/links: Add UX for updating any fields on an item that isn't in the selected repo
* Repo topics/links: Add UX for updating select fields of an item that isn't in the selected repo
* Fiddle with relay-router to avoid a blank page in some transitions
* Experiment with using borrows in some resolvers instead of copying
* Replace <Suspense>loading...</Suspense> with something nice
* Get UX working on mobile devices again
* Formulas and subscript and superscript in topic titles
* Add a docs repo and main page with screenshots and sections of how to do various things
* Make the display color repo-specific, and make the display color for Wiki to be transparent
* Investigate consolidating SearchMatch and TopicChild (TopicChildConnection)
* Drop need for Github login permission
* Drop need for Github email permission
* Repo selection UX: showing items
* Repo selection UX: updating items
* Decide whether the UX logic that is determining whether to show the "Edit" buttons should be moved into the meaning of viewerCanUpdate (e.g., whether a repo is selected)
* Decide whether the UX logic that determines whether the add link and add topic forms should be moved into the backend (e.g., when the topic is private and the selected repo is Wiki)
* Provide a guess as to what the stats are while they're being computed instead of return 0 topics and 0 links
* Use a queue for rebasing commits before mering, instead of retrying
* Look into using iterators for the search code instead of copying large buffers around
* Keep tabs on whether the ordering of changes is stable
* Get the topic upsert working when the topic already exists
* Replace multi-select with something that will work with relevance/contexts
* Show search input in input box when loaded from a url
* Find a new favicon / icon
* Replace delete link/topic confirmation popup with GitHub primer version
* Optimistic update for upserting and removing the topic timerange so that the UI updates quickly
* Consider using cypress for frontend integration testing
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
