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

      topic {
        id
        ...EditTopic_topic
      }
    }
  }
`
