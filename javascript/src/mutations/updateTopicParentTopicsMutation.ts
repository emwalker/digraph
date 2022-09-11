import { graphql } from 'react-relay'

export default graphql`
  mutation updateTopicParentTopicsMutation(
    $input: UpdateTopicParentTopicsInput!
  ) {
    updateTopicParentTopics(input: $input) {
      alerts {
        id
        text
        type
      }

      topic {
        ...Topic_topic
      }
    }
  }
`
