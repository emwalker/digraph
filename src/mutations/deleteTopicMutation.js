import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'

export default defaultMutation(graphql`
  mutation deleteTopicMutation(
    $input: DeleteTopicInput!
  ) {
    deleteTopic(input: $input) {
      clientMutationId
      deletedTopicId
    }
  }
`)
