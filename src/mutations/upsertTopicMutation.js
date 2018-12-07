import { graphql } from 'react-relay'

import defaultMutation from './defaultMutation'
import flashMessageUpdater from './flashMessageUpdater'

export default defaultMutation(graphql`
  mutation upsertTopicMutation(
    $input: UpsertTopicInput!
  ) {
    upsertTopic(input: $input) {
      alerts {
        text
        type
        id
      }

      topicEdge {
        node {
          ...Topic_topic
        }
      }
    }
  }
`, flashMessageUpdater('upsertTopic'))
