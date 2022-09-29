import { graphql } from 'react-relay'

export default graphql`
  mutation removeTopicTimerangeMutation(
    $input: RemoveTopicTimerangeInput!
  ) {
    removeTopicTimerange(input: $input) {
      clientMutationId

      updatedTopic {
        id
        displayName
      }

      updatedRepoTopic {
        topicId

        timerange {
          prefixFormat
          startsAt
        }
      }
    }
  }
`
