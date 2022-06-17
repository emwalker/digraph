Feature parity:
* Add in actor join clauses to all mutations.
* Fix upserting of users when a person logs in for the first time.
* Get user deletion working again
* Move the stats job to rust
* Get query info working again (the text and topics that appear in the search bar)
* Add user link reviews
* Go through rest of pageinfo test cases
* See if session problems go away if all sessions are deleted before switchover


Things that would be nice to eventually get to:
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
