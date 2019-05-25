// @flow
import { graphql } from 'react-relay'

import type { DeleteSynonymInput } from './__generated__/deleteSynonymMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = DeleteSynonymInput

export default defaultMutation(graphql`
  mutation deleteSynonymMutation(
    $input: DeleteSynonymInput!
  ) {
    deleteSynonym(input: $input) {
      clientMutationId
      deletedSynonymId

      alerts {
        id
        text
        type
      }

      topic {
        ...Topic_topic
        ...Synonyms_topic
      }
    }
  }
`, flashMessageUpdater('deleteSynonym'))
