import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'

export default defaultMutation(graphql`
  mutation updateLinkTopicsMutation(
    $input: UpdateLinkTopicsInput!
  ) {
    updateLinkTopics(input: $input) {
      link {
        ...Link_link
      }
    }
  }
`)
