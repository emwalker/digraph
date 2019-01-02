import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export default defaultMutation(graphql`
  mutation updateTopicMutation(
    $input: UpdateTopicInput!
  ) {
    updateTopic(input: $input) {
      alerts {
        id
        text
        type
      }

      topic {
        id
        name
        resourcePath
        description
      }
    }
  }
`, flashMessageUpdater('updateTopic'))
