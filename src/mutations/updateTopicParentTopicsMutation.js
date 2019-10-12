// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'
import type { UpdateTopicParentTopicsInput } from './__generated__/updateTopicParentTopicsMutation.graphql'

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
