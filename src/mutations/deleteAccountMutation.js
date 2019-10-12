// @flow
import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'
import type { DeleteAccountInput } from './__generated__/deleteAccountMutation.graphql'

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
