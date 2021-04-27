import { graphql } from 'react-relay'

import { DeleteTopicInput } from '__generated__/deleteTopicMutation.graphql'
import defaultMutation from './util/defaultMutation'

export type Input = DeleteTopicInput

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
