import { Environment, RecordSourceSelectorProxy } from 'relay-runtime'
import { commitMutation, graphql, DeclarativeMutationConfig } from 'react-relay'
import { v1 as uuidv1 } from 'uuid'

import type { UpsertLinkInput } from '__generated__/upsertLinkMutation.graphql'
import flashMessageUpdater from './util/flashMessageUpdater'
import updateTopicConnections from './util/updateTopicConnections'

export type Input = UpsertLinkInput

type Config = {
  configs: DeclarativeMutationConfig[],
}

let tmpId = 0

export default (environment: Environment, input: Input, config?: Config) => {
  const mutation = graphql`
    mutation upsertLinkMutation(
      $input: UpsertLinkInput!
    ) {
      upsertLink(input: $input) {
        alerts {
          text
          type
          id
        }

        linkEdge {
          node {
            ...Link_link
          }
        }
      }
    }
  `

  const optimisticUpdater = (store: RecordSourceSelectorProxy) => {
    tmpId += 1
    const id = `client:link:${tmpId}`
    const node = store.create(id, 'Link')
    node.setValue(id, 'id')
    node.setValue(input.title || 'Adding link to repo ...', 'title')
    node.setValue(input.url, 'url')
    node.setValue(true, 'loading')
    const parentTopicPaths = input.addParentTopicPaths || []
    updateTopicConnections(store, node, 'LinkEdge', parentTopicPaths, 'Topic_children')
  }

  return commitMutation(
    environment,
    {
      ...config,
      mutation,
      optimisticUpdater,
      updater: flashMessageUpdater('upsertLink'),
      variables: {
        input: { clientMutationId: uuidv1(), ...input },
      },
    },
  )
}
