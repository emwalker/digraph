import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'

export default defaultMutation(graphql`
  mutation updateTopicMutation(
    $input: UpdateTopicInput!
  ) {
    updateTopic(input: $input) {
      topic {
        id
        name
        resourcePath
        description
      }
    }
  }
`)
