import { graphql } from 'react-relay'

export default graphql`
  query EditLinkQuery(
    $linkId: ID!,
    $repoIds: [ID!],
    $viewerId: ID!,
  ) {
    view(
      repoIds: $repoIds,
      viewerId: $viewerId,
    ) {
      viewer {
        ...EditRepoLink_viewer
      }

      link(id: $linkId) {
        ...EditLink_link
      }
    }
  }
`
