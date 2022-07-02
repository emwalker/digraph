import { graphql } from 'react-relay'

import { DeleteTopicTimerangeInput } from '__generated__/deleteTopicTimerangeMutation.graphql'
import defaultMutation from './util/defaultMutation'

export type Input = DeleteTopicTimerangeInput

export default defaultMutation(graphql`
  mutation deleteTopicTimerangeMutation(
    $input: DeleteTopicTimerangeInput!
  ) {
    deleteTopicTimerange(input: $input) {
      clientMutationId

      topic {
        ...Topic_topic
        ...TopicTimerange_topic
      }
    }
  }
`)
