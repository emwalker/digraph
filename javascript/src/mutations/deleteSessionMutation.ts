import { graphql } from 'react-relay'

import { DeleteSessionInput } from '__generated__/deleteSessionMutation.graphql'
import defaultMutation from './util/defaultMutation'

export type Input = DeleteSessionInput

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
