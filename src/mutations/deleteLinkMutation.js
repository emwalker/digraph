// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import type { DeleteLinkInput } from './__generated__/deleteLinkMutation.graphql'

export type Input = DeleteLinkInput

export default defaultMutation(graphql`
  mutation deleteLinkMutation(
    $input: DeleteLinkInput!
  ) {
    deleteLink(input: $input) {
      clientMutationId
      deletedLinkId
    }
  }
`)
