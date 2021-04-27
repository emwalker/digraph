import { graphql } from 'react-relay'

import { UpdateLinkTopicsInput } from '__generated__/updateLinkTopicsMutation.graphql'
import defaultMutation from './util/defaultMutation'

export type Input = UpdateLinkTopicsInput

export default defaultMutation(graphql`
  mutation updateLinkTopicsMutation(
    $input: UpdateLinkTopicsInput!
  ) {
    updateLinkTopics(input: $input) {
      link {
        ...EditLinkForm_link
        ...Link_link
      }
    }
  }
`)
