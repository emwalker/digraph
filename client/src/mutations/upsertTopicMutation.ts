import { Dispatch, SetStateAction, useCallback } from 'react'
import { graphql, useMutation, ConnectionHandler } from 'react-relay'
import { RecordSourceSelectorProxy, RecordProxy } from 'relay-runtime'

import { AlertMessageType } from 'components/types'
import {
  upsertTopicMutation,
  upsertTopicMutation$data as ResponseType,
  OnMatchingSynonym as OnMatchingSynonymType,
} from '__generated__/upsertTopicMutation.graphql'

export type MatchingTopicsType = NonNullable<ResponseType['upsertTopic']>['matchingTopics']

function insertEdge(
  parentTopicId: string, store: RecordSourceSelectorProxy, topicEdge: RecordProxy<{}>,
) {
  const connectionId = ConnectionHandler.getConnectionID(parentTopicId,
    'ViewTopicPage_topic_children', { searchString: '' })

  if (!connectionId) {
    console.log('connection id not found for parent topic:', parentTopicId)
    return
  }

  const connection = store.get(connectionId)
  if (!connection) return

  ConnectionHandler.insertEdgeBefore(connection, topicEdge)
}

let tmpId = 0

function makeOptimisticUpdater(parentTopicId: string, displayName: string) {
  return (store: RecordSourceSelectorProxy) => {
    tmpId += 1
    const id = `client:topic:${tmpId}`
    const node = store.create(id, 'Topic')

    node.setValue(id, 'id')
    node.setValue(displayName, 'displayName')
    node.setValue(true, 'loading')
    node.setValue(null, 'repoTopics')
    node.setValue(false, 'showRepoOwnership')

    const topicEdge = store.create(`client:newEdge:${tmpId}`, 'TopicChildEdge')
    topicEdge.setLinkedRecord(node, 'node')

    return insertEdge(parentTopicId, store, topicEdge)
  }
}

function makeUpdater(parentTopicId: string) {
  return (store: RecordSourceSelectorProxy) => {
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

    return insertEdge(parentTopicId, store, topicEdge)
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

      updatedParentTopic {
        ...ViewTopicPage_topic
      }

      topicEdge {
        node {
          ...Topic_topic
        }
      }

      matchingTopics {
        displayName
        id

        displaySynonyms {
          name
          locale
        }

        ...UpdateTopic_updateTopic
      }
    }
  }
`

type Props<E> = {
  ignoreEvent?: (event: E) => boolean,
  makeAlert: (alert: AlertMessageType, matchingTopics: MatchingTopicsType) => void,
  name: string,
  onMatchingSynonym: OnMatchingSynonymType,
  parentTopicId: string,
  selectedRepoId: string | null,
  setName?: Dispatch<SetStateAction<string>>,
  updateTopicId?: string,
}

export function makeUpsertTopic<E>({
  ignoreEvent, makeAlert, name, onMatchingSynonym, selectedRepoId, setName, parentTopicId,
  updateTopicId,
}: Props<E>) {
  const upsertTopic = useMutation<upsertTopicMutation>(query)[0]

  const onCompleted = useCallback(((response: ResponseType) => {
    const alerts = response.upsertTopic?.alerts || []
    const matchingTopics = response.upsertTopic?.matchingTopics || []
    const addAlert = window.flashMessages?.addAlert
    if (addAlert == null) return

    for (const alert of alerts)
      addAlert(makeAlert(alert, matchingTopics))
  }), [makeAlert])

  return useCallback(
    (event: E) => {
      if (ignoreEvent && ignoreEvent(event)) return

      if (!selectedRepoId) {
        console.log('repo not selected')
        return
      }

      upsertTopic({
        onCompleted,
        updater: makeUpdater(parentTopicId),
        optimisticUpdater: makeOptimisticUpdater(parentTopicId, name),
        variables: {
          input: {
            name,
            onMatchingSynonym,
            parentTopicId,
            repoId: selectedRepoId,
            updateTopicId,
          },
        },
      })

      setName?.('')
    },
    [upsertTopic, selectedRepoId, parentTopicId, name, setName, makeUpdater, makeOptimisticUpdater],
  )
}
