import { commitMutation, graphql } from 'react-relay'
import uuidv1 from 'uuid/v1'

const mutation = graphql`
  mutation selectTopicMutation(
    $input: SelectTopicInput!
  ) {
    selectTopic(input: $input) {
      topic {
        name
        resourcePath
      }
    }
  }
`

export default (environment, input) => {
  const clientMutationId = uuidv1()
  return commitMutation(
    environment,
    {
      mutation,
      variables: {
        input: { clientMutationId, ...input },
      },
    },
  )
}
