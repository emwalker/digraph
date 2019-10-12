// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import type { UpsertTopicTimeRangeInput } from './__generated__/deleteTopicTimeRangeMutation.graphql'

export type Input = UpsertTopicTimeRangeInput

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
