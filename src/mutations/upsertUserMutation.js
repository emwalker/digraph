import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export default defaultMutation(graphql`
  mutation upsertUserMutation(
    $input: UpsertUserInput!
  ) {
    upsertUser(input: $input) {
      alerts {
        id
        text
        type
      }

      userEdge {
        node {
          id
        }
      }
    }
  }
`, flashMessageUpdater('upsertUser'))
