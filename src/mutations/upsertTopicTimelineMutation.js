import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export default defaultMutation(graphql`
  mutation upsertTopicTimelineMutation(
    $input: UpsertTopicTimelineInput!
  ) {
    upsertTopicTimeline(input: $input) {
      alerts {
        id
        text
        type
      }

      topic {
        ...TopicTimeline_topic
      }
    }
  }
`, flashMessageUpdater('upsertTopicTimeline'))
