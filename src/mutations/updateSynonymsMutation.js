// @flow
import { graphql } from 'react-relay'

import type { UpdateSynonymsInput } from './__generated__/updateSynonymsMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = UpdateSynonymsInput

export default defaultMutation(graphql`
  mutation updateSynonymsMutation(
    $input: UpdateSynonymsInput!
  ) {
    updateSynonyms(input: $input) {
      clientMutationId

      alerts {
        id
        text
        type
      }

      topic {
        displayName(timeRange: true)
        ...Synonyms_topic
      }
    }
  }
`, flashMessageUpdater('updateSynonyms'))
