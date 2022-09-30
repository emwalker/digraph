import { graphql } from 'react-relay'

export default graphql`
  query RepoTopicParentTopicsRefetchQuery(
    $searchString: String!,
    $selectedRepoId: ID!,
    $topicId: ID!,
    $viewerId: ID!,
  ) {
    view(viewerId: $viewerId) {
      topic(id: $topicId) {
        repoTopic(repoId: $selectedRepoId) {
          availableParentTopics(searchString: $searchString) {
            synonymMatches {
              value: id
              label: displayName
            }
          }
        }
      }
    }
  }
`
