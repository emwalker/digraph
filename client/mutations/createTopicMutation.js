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
      key: 'Organization_topics',
      rangeBehavior: 'append',
    }],
    edgeName: 'topicEdge',
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
