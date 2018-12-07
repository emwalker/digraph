import { graphql } from 'react-relay'

import defaultMutation from './defaultMutation'
import flashMessageUpdater from './flashMessageUpdater'

export default defaultMutation(graphql`
  mutation updateTopicParentTopicsMutation(
    $input: UpdateTopicParentTopicsInput!
  ) {
    updateTopicParentTopics(input: $input) {
      alerts {
        id
        text
        type
      }

      topic {
        ...Topic_topic
      }
    }
  }
`, flashMessageUpdater('updateTopicParentTopics'))
