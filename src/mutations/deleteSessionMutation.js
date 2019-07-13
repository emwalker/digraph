import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'

export default defaultMutation(graphql`
  mutation deleteSessionMutation(
    $input: DeleteSessionInput!
  ) {
    deleteSession(input: $input) {
      clientMutationId
      deletedSessionId
    }
  }
`)
