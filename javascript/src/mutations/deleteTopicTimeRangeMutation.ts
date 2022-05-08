import { graphql } from 'react-relay'

import { DeleteTopicTimeRangeInput } from '__generated__/deleteTopicTimeRangeMutation.graphql'
import defaultMutation from './util/defaultMutation'

export type Input = DeleteTopicTimeRangeInput

export default defaultMutation(graphql`
  mutation deleteTopicTimeRangeMutation(
    $input: DeleteTopicTimeRangeInput!
  ) {
    deleteTopicTimeRange(input: $input) {
      clientMutationId

      topic {
        ...Topic_topic
        ...TopicTimeRange_topic
      }
    }
  }
`)
