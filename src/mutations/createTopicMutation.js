import { graphql } from 'react-relay'

import defaultMutation from './defaultMutation'

export default defaultMutation(graphql`
  mutation createTopicMutation(
    $input: CreateTopicInput!
  ) {
    createTopic(input: $input) {
      topicEdge {
        node {
          ...Topic_topic
        }
      }
    }
  }
`)
