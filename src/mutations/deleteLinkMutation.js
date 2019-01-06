import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'

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
