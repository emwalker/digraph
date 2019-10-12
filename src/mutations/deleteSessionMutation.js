// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import type { DeleteSessionInput } from './__generated__/deleteSessionMutation.graphql'

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
