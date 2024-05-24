import { graphql } from 'react-relay'

export default graphql`
  mutation deleteAccountMutation(
    $input: DeleteAccountInput!
  ) {
    deleteAccount(input: $input) {
      clientMutationId
      deletedUserId

      alerts {
        id
        text
        type
      }
    }
  }
`
