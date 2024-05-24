import { graphql } from 'react-relay'

export default graphql`
  mutation selectRepositoryMutation(
    $input: SelectRepositoryInput!
  ) {
    selectRepository(input: $input) {
      viewer {
        ...AddForm_viewer
      }

      repo {
        id
        isPrivate
      }

      currentTopic {
        ...ViewTopicPage_topic
      }
    }
  }
`
