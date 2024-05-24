import { graphql } from 'react-relay'

export default graphql`
  mutation deleteSessionMutation(
    $input: DeleteSessionInput!
  ) {
    deleteSession(input: $input) {
      clientMutationId
      deletedSessionId
    }
  }
`
