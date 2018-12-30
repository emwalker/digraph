import { commitMutation, graphql } from 'react-relay'
import uuidv1 from 'uuid/v1'

import flashMessageUpdater from './util/flashMessageUpdater'
import updateTopicConnections from './util/updateTopicConnections'

let tmpId = 0

export default (environment, configs, input) => {
  const mutation = graphql`
  mutation upsertTopicMutation(
      $input: UpsertTopicInput!
    ) {
      upsertTopic(input: $input) {
        alerts {
          text
          type
          id
        }

        topicEdge {
          node {
            ...Topic_topic
          }
        }
      }
    }
  `

  const optimisticUpdater = (store) => {
    tmpId += 1
    const id = `client:topic:${tmpId}`
    const node = store.create(id, 'Topic')
    node.setValue(id, 'id')
    node.setValue(input.name, 'name')
    node.setValue('Adding topic to the repo ...', 'description')
    updateTopicConnections(store, node, 'TopicEdge', input.topicIds || [], 'Topic_childTopics')
  }

  return commitMutation(
    environment,
    {
      mutation,
      configs,
      optimisticUpdater,
      updater: flashMessageUpdater('upsertTopic'),
      variables: {
        input: { clientMutationId: uuidv1(), ...input },
      },
    },
  )
}
