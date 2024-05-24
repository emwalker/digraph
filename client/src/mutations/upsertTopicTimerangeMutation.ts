import { graphql } from 'react-relay'

export default graphql`
  mutation upsertTopicTimerangeMutation(
    $input: UpsertTopicTimerangeInput!
  ) {
    upsertTopicTimerange(input: $input) {
      alerts {
        id
        text
        type
      }

      updatedTopic {
        id
        ...EditTopic_topic
      }

      updatedRepoTopic {
        id
        ...RepoTopicTimerange_repoTopic
      }
    }
  }
`
