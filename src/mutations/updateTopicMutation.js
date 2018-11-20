import { commitMutation, graphql } from 'react-relay'
import uuidv1 from 'uuid/v1'

const mutation = graphql`
  mutation updateTopicMutation(
    $input: UpdateTopicInput!
  ) {
    updateTopic(input: $input) {
      topic {
        id
        name
        resourcePath
        description
      }
    }
  }
`

export default (environment, configs, input) => {
  const clientMutationId = uuidv1()

  return commitMutation(
    environment,
    {
      mutation,
      configs,
      variables: {
        input: { clientMutationId, ...input },
      },
    },
  )
}
