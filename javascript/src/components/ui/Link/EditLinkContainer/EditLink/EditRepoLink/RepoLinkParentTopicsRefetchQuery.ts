import { graphql } from 'react-relay'

export default graphql`
  query RepoLinkParentTopicsRefetchQuery(
    $linkId: ID!,
    $searchString: String!,
    $selectedRepoId: ID!,
    $viewerId: ID!,
  ) {
    view(viewerId: $viewerId) {
      link(id: $linkId) {
        repoLink(repoId: $selectedRepoId) {
          availableParentTopics(searchString: $searchString) {
            synonyms {
              value: id
              label: displayName
            }
          }
        }
      }
    }
  }
`
