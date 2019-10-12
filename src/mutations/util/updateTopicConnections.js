// @flow
import { type Node } from 'react'
import { ConnectionHandler } from 'relay-runtime'

let tmpId = 0

export default (
  store: any,
  node: Node,
  edgeType: Object,
  topicIds: $ReadOnlyArray<string>,
  connectionName: string,
) => {
  tmpId += 1

  const newEdge = store.create(`client:newEdge:${tmpId}`, edgeType)
  newEdge.setLinkedRecord(node, 'node')

  topicIds.forEach((topicId) => {
    const topicProxy = store.get(topicId)
    const conn = ConnectionHandler.getConnection(topicProxy, connectionName)
    ConnectionHandler.insertEdgeBefore(conn, newEdge)
  })
}
