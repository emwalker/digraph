import { graphql } from 'react-relay'

import { UpdateTopicInput } from '__generated__/updateTopicMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = UpdateTopicInput

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
