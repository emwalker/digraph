import { commitMutation, graphql } from 'react-relay'
import uuidv1 from 'uuid/v1'

const mutation = graphql`
  mutation updateLinkTopicsMutation(
    $input: UpdateLinkTopicsInput!
  ) {
    updateLinkTopics(input: $input) {
      link {
        parentTopics(first: 100) {
          edges {
            node {
              id
            }
          }
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
