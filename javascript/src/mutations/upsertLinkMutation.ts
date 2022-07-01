import { ConnectionHandler, Environment, RecordProxy, RecordSourceSelectorProxy } from 'relay-runtime'
import { commitMutation, graphql, DeclarativeMutationConfig } from 'react-relay'
import { v1 as uuidv1 } from 'uuid'

import type { UpsertLinkInput } from '__generated__/upsertLinkMutation.graphql'
import flashMessageUpdater from './util/flashMessageUpdater'

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
            id
            ...Link_link
          }
        }
      }
    }
  `

  const insertLink = (conn: RecordProxy, edge: RecordProxy, itemId: string) => {
    const prevEdges = conn.getLinkedRecords('edges')
    if (!prevEdges) return

    let node = prevEdges.map((e) => e.getLinkedRecord('node'))
      .find((n) => n?.getValue('id') == itemId)

    if (node) {
      console.log('link found, skipping')
      return
    }

    console.log('adding new link')
    // Find first link; we'll insert before it
    // https://github.com/facebook/relay/issues/2761#issuecomment-501552410
    let firstLinkIndex = prevEdges.map((e) => e.getLinkedRecord('node'))
      .findIndex((n) => n?.getValue('__typename') == 'Link')

    if (!firstLinkIndex) {
      ConnectionHandler.insertEdgeAfter(conn, edge)
      return
    }

    prevEdges.splice(firstLinkIndex, 0, edge)
    conn.setLinkedRecords(prevEdges, 'edges')
  }

  const optimisticUpdater = (store: RecordSourceSelectorProxy) => {
    tmpId += 1
    const parentTopicPaths = input.addParentTopicPaths || []
    const nodeId = `client:link:${tmpId}`
    const node = store.create(nodeId, 'Link')

    node.setValue(nodeId, 'id')
    node.setValue(input.title || 'Adding link to repo ...', 'title')
    node.setValue(input.url, 'url')
    node.setValue(true, 'loading')

    for (const path of parentTopicPaths) {
      const topicProxy = store.get(path)
      if (!topicProxy) continue

      const conn = ConnectionHandler.getConnection(topicProxy, 'Topic_children')
      if (!conn) continue

      const edge = store.create(`client:newEdge:${tmpId}`, 'TopicChildEdge')
      edge.setLinkedRecord(node, 'node')

      insertLink(conn, edge, nodeId)
    }
  }

  const updater = (store: RecordSourceSelectorProxy) => {
    const payload = store.getRootField('upsertLink')
    const edge = payload?.getLinkedRecord('linkEdge')
    const link = edge?.getLinkedRecord('node')
    const linkId = link?.getValue('id') as string

    if (!edge) return
    if (!linkId) return

    const parentTopicPaths = input.addParentTopicPaths || []

    for (const path of parentTopicPaths) {
      const topicProxy = store.get(path)
      if (!topicProxy) continue

      const conn = ConnectionHandler.getConnection(topicProxy, 'Topic_children')
      if (!conn) continue

      insertLink(conn, edge, linkId)
    }

    flashMessageUpdater('upsertLink')
  }

  return commitMutation(
    environment,
    {
      ...config,
      mutation,
      optimisticUpdater,
      updater,
      variables: {
        input: { clientMutationId: uuidv1(), ...input },
      },
    },
  )
}
