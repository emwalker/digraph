import { Dispatch, SetStateAction, useCallback, KeyboardEvent } from 'react'
import { graphql, useMutation, ConnectionHandler } from 'react-relay'
import { RecordSourceSelectorProxy } from 'relay-runtime'
import { upsertTopicMutation } from '__generated__/upsertTopicMutation.graphql'

function makeUpdater(parentTopicId: string) {
  return (store: RecordSourceSelectorProxy) => {
    const connectionId = ConnectionHandler.getConnectionID(parentTopicId,
      'ViewTopicPage_topic_children', { searchString: '' })

    if (!connectionId) {
      console.log('connection id not found for parent topic:', parentTopicId)
      return
    }

    const connection = store.get(connectionId)
    if (!connection) {
      console.log('connection not found for id:', connectionId)
      return
    }

    const payload = store.getRootField('upsertTopic')
    if (!payload) {
      console.log('payload not found in mutation response')
      return
    }

    const topicEdge = payload.getLinkedRecord('topicEdge')
    if (!topicEdge) {
      console.log('no topic edge found in mutation response')
      return
    }

    ConnectionHandler.insertEdgeBefore(connection, topicEdge)
  }
}

const query = graphql`
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

export function makeUpsertTopic({ selectedRepoId, name, setName, topicId }: {
  name: string,
  selectedRepoId: string | null,
  setName: Dispatch<SetStateAction<string>>,
  topicId: string,
}) {
  const upsertTopic = useMutation<upsertTopicMutation>(query)[0]

  return useCallback((event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key !== 'Enter') return

    if (!selectedRepoId) {
      // eslint-disable-next-line no-console
      console.log('repo not selected')
      return
    }

    upsertTopic({
      variables: {
        input: {
          name,
          repoId: selectedRepoId,
          parentTopicId: topicId,
        },
      },
      updater: makeUpdater(topicId),
    })

    setName('')
  }, [upsertTopic, selectedRepoId, topicId, name, setName])
}
