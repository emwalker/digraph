import { commitMutation, graphql } from 'react-relay'
import uuidv1 from 'uuid/v1'

const mutation = graphql`
  mutation upsertLinkMutation(
    $input: UpsertLinkInput!
  ) {
    upsertLink(input: $input) {
      linkEdge {
        node {
          ...Link_link
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
