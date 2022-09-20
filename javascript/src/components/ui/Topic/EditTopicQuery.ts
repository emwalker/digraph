import { graphql } from 'react-relay'

export default graphql`
  query EditTopicQuery(
    $repoIds: [ID!],
    $topicId: String!,
    $viewerId: ID!,
  ) {
    view(
      repoIds: $repoIds,
      viewerId: $viewerId,
    ) {
      viewer {
        ...EditRepoTopic_viewer
      }

      topic(id: $topicId) {
        ...EditTopic_topic
      }
    }
  }
`
