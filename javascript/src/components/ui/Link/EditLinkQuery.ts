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
        ...EditLink_viewer
      }

      link(id: $linkId) {
        ...EditLink_link
      }
    }
  }
`
