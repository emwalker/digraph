import { graphql } from 'react-relay'

export default graphql`
  mutation deleteTopicMutation(
    $input: DeleteTopicInput!
  ) {
    deleteTopic(input: $input) {
      clientMutationId
      deletedTopicId
    }
  }
`