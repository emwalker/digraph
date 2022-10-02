import { Dispatch, KeyboardEvent, SetStateAction, useCallback } from 'react'
import { ConnectionHandler, graphql, useMutation } from 'react-relay'
import { RecordSourceSelectorProxy } from 'relay-runtime'

import { upsertLinkMutation } from '__generated__/upsertLinkMutation.graphql'

function makeUpdater(parentTopicId: string | null) {
  if (!parentTopicId) return null

  return (store: RecordSourceSelectorProxy) => {
    const connectionId = ConnectionHandler.getConnectionID(parentTopicId,
      'ViewTopicPage_topic_children', { searchString: '' })
    if (!connectionId) {
      console.log('no connection id found under topic:', parentTopicId)
      return
    }

    const connection = store.get(connectionId)
    if (!connection) {
      console.log('no connection found for id:', connectionId)
      return
    }

    const payload = store.getRootField('upsertLink')
    if (!payload) {
      console.log('payload not found in mutation response')
      return
    }

    const linkEdge = payload.getLinkedRecord('linkEdge')
    if (!linkEdge) {
      console.log('no topic edge found in mutation response')
      return
    }

    ConnectionHandler.insertEdgeBefore(connection, linkEdge)
  }
}

const query = graphql`
  mutation upsertLinkMutation($input: UpsertLinkInput!) {
    upsertLink(input: $input) {
      alerts {
        text
        type
        id
      }

      linkEdge {
        node {
          displayTitle
          displayUrl
          id
          loading
          newlyAdded
          viewerCanUpdate
        }
      }
    }
  }
`

type Props = {
  linkId?: string | null,
  selectedRepoId: string | null,
  setUrl?: Dispatch<SetStateAction<string>>,
  title?: string | null,
  topicId?: string | null,
  url: string,
}

export function makeUpsertLinkCallback({
  linkId, selectedRepoId, setUrl, title, topicId, url,
}: Props) {
  const [upsertLink, upsertLinkInFlight] = useMutation<upsertLinkMutation>(query)

  return useCallback(() => {
    if (upsertLinkInFlight) {
      console.log('mutation already in flight')
      return
    }

    if (!selectedRepoId) {
      console.log('repo not selected')
      return
    }

    const displayTitle = title || 'Fetching page title ...'

    const optimisticResponse = {
      upsertLink: {
        alerts: [],
        linkEdge: {
          node: {
            displayTitle,
            displayUrl: url,
            id: linkId || `client:links:${Math.random()}`,
            loading: true,
            newlyAdded: linkId == null,
            viewerCanUpdate: true,
          },
        },
      },
    }

    const updater = makeUpdater(topicId || null)

    upsertLink({
      variables: {
        input: {
          addParentTopicId: topicId,
          linkId,
          repoId: selectedRepoId,
          title,
          url,
        },
      },
      updater,
      optimisticUpdater: updater,
      optimisticResponse,
    })

    if (setUrl) setUrl('')
  }, [upsertLink, selectedRepoId, topicId, url, setUrl, title])
}
