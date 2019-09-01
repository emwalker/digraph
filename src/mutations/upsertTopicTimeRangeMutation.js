import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

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
