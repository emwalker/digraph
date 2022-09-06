import { graphql } from 'react-relay'

import { UpdateTopicSynonymsInput } from '__generated__/updateTopicSynonymsMutation.graphql'
import defaultMutation from './util/defaultMutation'
import flashMessageUpdater from './util/flashMessageUpdater'

export type Input = UpdateTopicSynonymsInput

const updateTopicSynonymsMutation = defaultMutation(graphql`
  mutation updateTopicSynonymsMutation(
    $input: UpdateTopicSynonymsInput!
  ) {
    updateTopicSynonyms(input: $input) {
      clientMutationId

      alerts {
        id
        text
        type
      }

      topic {
        displayName
        ...Synonyms_topic
      }
    }
  }
`, flashMessageUpdater('updateTopicSynonyms'))

export default updateTopicSynonymsMutation
