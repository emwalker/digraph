import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'

export default defaultMutation(graphql`
  mutation deleteTopicTimeRangeMutation(
    $input: DeleteTopicTimeRangeInput!
  ) {
    deleteTopicTimeRange(input: $input) {
      clientMutationId

      topic {
        ...Topic_topic
        ...TopicTimeRange_topic
      }
    }
  }
`)
