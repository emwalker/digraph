import { graphql } from 'react-relay'

import { UpdateTopicParentTopicsInput } from '__generated__/updateTopicParentTopicsMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = UpdateTopicParentTopicsInput

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
