// @flow
import { graphql } from 'react-relay'

import type { AddSynonymInput } from './__generated__/addSynonymMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = AddSynonymInput

export default defaultMutation(graphql`
  mutation addSynonymMutation(
    $input: AddSynonymInput!
  ) {
    addSynonym(input: $input) {
      clientMutationId

      alerts {
        id
        text
        type
      }

      synonymEdge {
        node {
          name
        }
      }

      topic {
        ...Topic_topic
        ...Synonyms_topic
      }
    }
  }
`, flashMessageUpdater('addSynonym'))
