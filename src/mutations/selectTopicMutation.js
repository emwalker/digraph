import { graphql } from 'react-relay'

import defaultMutation from './defaultMutation'

export default defaultMutation(graphql`
  mutation selectTopicMutation(
    $input: SelectTopicInput!
  ) {
    selectTopic(input: $input) {
      topic {
        name
        resourcePath
      }
    }
  }
`)
