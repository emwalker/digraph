import { graphql } from 'react-relay'

export default graphql`
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

      updatedTopic {
        id
        displayName
      }

      updatedRepoTopic {
        id
        ...RepoTopicSynonyms_repoTopic
      }
    }
  }
`
