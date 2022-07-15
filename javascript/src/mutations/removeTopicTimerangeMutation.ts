import { graphql } from 'react-relay'

import { RemoveTopicTimerangeInput } from '__generated__/removeTopicTimerangeMutation.graphql'
import defaultMutation from './util/defaultMutation'

export type Input = RemoveTopicTimerangeInput

export default defaultMutation(graphql`
  mutation removeTopicTimerangeMutation(
    $input: RemoveTopicTimerangeInput!
  ) {
    removeTopicTimerange(input: $input) {
      clientMutationId

      topic {
        ...Topic_topic
        ...TopicTimerange_topic
      }
    }
  }
`)
