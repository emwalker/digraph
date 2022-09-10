import { Environment } from 'react-relay'
import { RecordSourceSelectorProxy } from 'relay-runtime'
import { commitMutation, graphql, DeclarativeMutationConfig } from 'react-relay'
import { v1 as uuidv1 } from 'uuid'

import { UpsertTopicInput as Input } from '__generated__/upsertTopicMutation.graphql'
import flashMessageUpdater from './util/flashMessageUpdater'
import updateTopicConnections from './util/updateTopicConnections'

type Config = {
  configs: DeclarativeMutationConfig[],
}

let tmpId = 0

export default (environment: Environment, input: Input, config: Config) => {
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

  const optimisticUpdater = (store: RecordSourceSelectorProxy) => {
    tmpId += 1
    const id = `client:topic:${tmpId}`
    const node = store.create(id, 'Topic')
    node.setValue(id, 'id')
    node.setValue(input.name, 'name')
    node.setValue('Adding topic to the repo ...', 'description')
    node.setValue(true, 'loading')
    updateTopicConnections(store, node, 'TopicChildEdge', input.parentTopicId, 'Topic_children')
  }

  return commitMutation(
    environment,
    {
      ...config,
      mutation,
      optimisticUpdater,
      updater: flashMessageUpdater('upsertTopic'),
      variables: {
        input: { clientMutationId: uuidv1(), ...input },
      },
    },
  )
}
