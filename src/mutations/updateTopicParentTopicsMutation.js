import { graphql } from 'react-relay'

import defaultMutation from './defaultMutation'

export default defaultMutation(graphql`
  mutation updateTopicParentTopicsMutation(
    $input: UpdateTopicParentTopicsInput!
  ) {
    updateTopicParentTopics(input: $input) {
      topic {
        parentTopics(first: 100) {
          edges {
            node {
              id
            }
          }
        }
      }
    }
  }
`)
