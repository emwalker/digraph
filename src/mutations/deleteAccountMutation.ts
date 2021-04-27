import { graphql } from 'react-relay'

import { DeleteAccountInput } from '__generated__/deleteAccountMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = DeleteAccountInput

export default defaultMutation(graphql`
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
`, flashMessageUpdater('deleteAccount'))
