// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import type { DeleteTopicInput } from './__generated__/deleteTopicMutation.graphql'

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
