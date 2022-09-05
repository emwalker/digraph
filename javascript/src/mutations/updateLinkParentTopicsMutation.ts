import { graphql } from 'react-relay'

import { UpdateLinkParentTopicsInput } from '__generated__/updateLinkParentTopicsMutation.graphql'
import defaultMutation from './util/defaultMutation'

export type Input = UpdateLinkParentTopicsInput

export default defaultMutation(graphql`
  mutation updateLinkParentTopicsMutation(
    $input: UpdateLinkParentTopicsInput!
  ) {
    updateLinkParentTopics(input: $input) {
      link {
        details {
          ...EditLinkForm_linkDetail
        }
        ...Link_link
      }
    }
  }
`)
