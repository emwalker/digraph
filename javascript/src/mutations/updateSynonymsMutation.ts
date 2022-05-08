import { graphql } from 'react-relay'

import { UpdateSynonymsInput } from '__generated__/updateSynonymsMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = UpdateSynonymsInput

const updateSynonymsMutation = defaultMutation(graphql`
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
        displayName: name
        ...Synonyms_topic
      }
    }
  }
`, flashMessageUpdater('updateSynonyms'))

export default updateSynonymsMutation
