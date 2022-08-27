import { graphql } from 'react-relay'

import { DeleteLinkInput } from '__generated__/deleteLinkMutation.graphql'
import defaultMutation from './util/defaultMutation'

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
