import { graphql } from 'react-relay'

import defaultMutation from './defaultMutation'
import flashMessageUpdater from './flashMessageUpdater'

export default defaultMutation(graphql`
  mutation upsertLinkMutation(
    $input: UpsertLinkInput!
  ) {
    upsertLink(input: $input) {
      alerts {
        text
        type
        id
      }

      linkEdge {
        node {
          ...Link_link
        }
      }
    }
  }
`, flashMessageUpdater('upsertLink'))
