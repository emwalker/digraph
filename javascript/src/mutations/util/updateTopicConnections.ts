import { ConnectionHandler, RecordSourceSelectorProxy, RecordProxy } from 'relay-runtime'

let tmpId = 0

export default (
  store: RecordSourceSelectorProxy,
  node: RecordProxy,
  edgeType: string,
  topicIds: readonly string[],
  connectionName: string,
) => {
  tmpId += 1

  const newEdge = store.create(`client:newEdge:${tmpId}`, edgeType)
  newEdge.setLinkedRecord(node, 'node')

  topicIds.forEach((topicId) => {
    const topicProxy = store.get(topicId)
    if (topicProxy) {
      const conn = ConnectionHandler.getConnection(topicProxy, connectionName)
      if (conn) ConnectionHandler.insertEdgeBefore(conn, newEdge)
    }
  })
}
