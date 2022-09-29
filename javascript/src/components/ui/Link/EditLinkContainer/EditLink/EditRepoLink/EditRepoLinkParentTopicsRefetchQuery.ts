import { graphql } from 'react-relay'

export default graphql`
  query EditRepoLinkParentTopicsRefetchQuery(
    $linkId: ID!,
    $selectedRepoId: ID!,
    $viewerId: ID!,
  ) {
    view(viewerId: $viewerId) {
      link(id: $linkId) {
        repoLink(repoId: $selectedRepoId) {
          ...EditRepoLinkParentTopics_repoLink
        }
      }
    }
  }
`
