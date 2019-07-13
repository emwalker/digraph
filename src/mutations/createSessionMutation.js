import { graphql } from 'react-relay'

import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export default defaultMutation(graphql`
  mutation createSessionMutation(
    $input: CreateSessionInput!
  ) {
    createSession(input: $input) {
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

      sessionEdge {
        node {
          id
        }
      }
    }
  }
`, flashMessageUpdater('createSession'))
