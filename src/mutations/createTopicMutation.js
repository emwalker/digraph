import { commitMutation, graphql } from 'react-relay'
import uuidv1 from 'uuid/v1'

const mutation = graphql`
  mutation createTopicMutation(
    $input: CreateTopicInput!
  ) {
    createTopic(input: $input) {
      topicEdge {
        node {
          id
          name
          resourceId
          resourcePath
        }
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
