// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'
import type { UpsertTopicTimeRangeInput } from './__generated__/upsertTopicTimeRangeMutation.graphql'

export type Input = UpsertTopicTimeRangeInput

export default defaultMutation(graphql`
  mutation upsertTopicTimeRangeMutation(
    $input: UpsertTopicTimeRangeInput!
  ) {
    upsertTopicTimeRange(input: $input) {
      alerts {
        id
        text
        type
      }

      topic {
        ...Topic_topic
        ...TopicTimeRange_topic
      }
    }
  }
`, flashMessageUpdater('upsertTopicTimeRange'))
