import { graphql } from 'react-relay'

import { UpsertTopicTimerangeInput } from '__generated__/upsertTopicTimerangeMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = UpsertTopicTimerangeInput

export default defaultMutation(graphql`
  mutation upsertTopicTimerangeMutation(
    $input: UpsertTopicTimerangeInput!
  ) {
    upsertTopicTimerange(input: $input) {
      alerts {
        id
        text
        type
      }

      topic {
        ...Topic_topic

        details {
          ...TopicTimerange_topicDetail
        }
      }
    }
  }
`, flashMessageUpdater('upsertTopicTimerange'))
