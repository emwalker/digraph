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

      topic {
        id
        displayName

        repoTopics {
          id
          ...RepoTopicSynonyms_repoTopic
        }
      }
    }
  }
`
