import { graphql } from 'react-relay'

export default graphql`
  mutation removeTopicTimerangeMutation(
    $input: RemoveTopicTimerangeInput!
  ) {
    removeTopicTimerange(input: $input) {
      clientMutationId

      topic {
        ...Topic_topic

        repoTopics {
          ...RepoTopicTimerange_repoTopic
        }
      }
    }
  }
`
