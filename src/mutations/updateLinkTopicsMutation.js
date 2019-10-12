// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import type { UpdateLinkTopicsInput } from './__generated__/updateLinkTopicsMutation.graphql'

export type Input = UpdateLinkTopicsInput

export default defaultMutation(graphql`
  mutation updateLinkTopicsMutation(
    $input: UpdateLinkTopicsInput!
  ) {
    updateLinkTopics(input: $input) {
      link {
        ...EditLink_link
        ...Link_link
      }
    }
  }
`)
