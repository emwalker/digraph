import { graphql } from 'react-relay'

export default graphql`
  query EditTopicQuery(
    $viewerId: ID!,
    $repoIds: [ID!],
    $topicId: String!,
  ) {
    view(
      viewerId: $viewerId,
      repositoryIds: $repoIds,
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
