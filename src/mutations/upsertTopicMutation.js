// @flow
import { Environment } from 'relay-runtime'
import { commitMutation, graphql } from 'react-relay'
import { v1 as uuidv1 } from 'uuid'

import flashMessageUpdater from './util/flashMessageUpdater'
import updateTopicConnections from './util/updateTopicConnections'
import type { UpdateTopicInput as Input } from './__generated__/upsertTopicMutation.graphql'

type Config = {|
  configs: Array<*>,
|}

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

  const optimisticUpdater = (store) => {
    tmpId += 1
    const id = `client:topic:${tmpId}`
    const node = store.create(id, 'Topic')
    node.setValue(id, 'id')
    node.setValue(input.name, 'name')
    node.setValue('Adding topic to the repo ...', 'description')
    node.setValue(true, 'loading')
    updateTopicConnections(store, node, 'TopicEdge', input.topicIds || [], 'Topic_childTopics')
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
