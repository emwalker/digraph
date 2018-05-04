import { commitMutation, graphql } from 'react-relay'
import uuidv1 from 'uuid/v1'

const mutation = graphql`
  mutation createLinkMutation(
    $input: CreateLinkInput!
  ) {
    createLink(input: $input) {
      linkEdge {
        node {
          id
          resourceId
          resourcePath
          title
          url

          topics(first: 5) {
            edges {
              node {
                name
                resourceId
                resourcePath
              }
            }
          }
        }
      }
    }
  }
`

export default (environment, organizationId, input) => {
  const configs = [{
    type: 'RANGE_ADD',
    parentID: organizationId,
    connectionInfo: [{
      key: 'Organization_links',
      rangeBehavior: 'append',
    }],
    edgeName: 'linkEdge',
  }]

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
