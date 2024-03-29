import { Dispatch, SetStateAction, useCallback } from 'react'
import { ConnectionHandler, graphql, useMutation } from 'react-relay'
import { RecordProxy, RecordSourceSelectorProxy } from 'relay-runtime'

import { showAlerts } from 'components/helpers'
import {
  upsertLinkMutation,
  upsertLinkMutation$data as ResponseType,
} from '__generated__/upsertLinkMutation.graphql'

function hasErrors(payload: RecordProxy<{}>) {
  const errors = (payload.getLinkedRecords('alerts') || []).filter((alert: RecordProxy<{}>) => {
    return alert.getValue('type') == 'ERROR'
  })

  if (errors.length !== 0) {
    console.log('errors present, not updating link connection')
    return true
  }

  return false
}

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
    if (!connection) return

    const payload = store.getRootField('upsertLink')
    if (!payload) {
      console.log('payload not found in mutation response')
      return
    }

    if (hasErrors(payload)) return

    const linkEdge = payload.getLinkedRecord('linkEdge')
    if (!linkEdge) {
      console.log('no topic edge found in mutation response')
      return
    }

    const prevEdges = connection.getLinkedRecords('edges') || []
    const index = prevEdges.findIndex((edge) => {
      const node = edge.getLinkedRecord('node')
      return node?.getValue('__typename') == 'Link'
    })
    prevEdges.splice(index, 0, linkEdge)
    connection.setLinkedRecords(prevEdges, 'edges')
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
          ...Link_link
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
  url: string | null,
}

export function makeUpsertLinkCallback({
  linkId, selectedRepoId, setUrl, title, topicId, url,
}: Props) {
  const [upsertLink, upsertLinkInFlight] = useMutation<upsertLinkMutation>(query)
  const onCompleted = showAlerts((response: ResponseType) => response.upsertLink?.alerts || [])

  return useCallback(() => {
    if (upsertLinkInFlight) {
      console.log('mutation already in flight')
      return
    }

    if (!selectedRepoId) {
      console.log('repo not selected')
      return
    }

    if (!url) {
      console.log('no url')
      return
    }

    const displayTitle = title || 'Fetching link title ...'

    const optimisticResponse = {
      upsertLink: {
        alerts: [],
        linkEdge: {
          node: {
            displayParentTopics: { edges: [] },
            displayTitle,
            displayUrl: url,
            id: linkId || `client:links:${Math.random()}`,
            loading: true,
            newlyAdded: linkId == null,
            repoLinks: [],
            showRepoOwnership: false,
            viewerCanUpdate: false,
          },
        },
      },
    }

    const updater = makeUpdater(topicId || null)

    upsertLink({
      onCompleted,
      updater,
      optimisticUpdater: updater,
      optimisticResponse,
      variables: {
        input: {
          addParentTopicId: topicId,
          linkId,
          repoId: selectedRepoId,
          title,
          url,
        },
      },
    })

    if (setUrl) setUrl('')
  }, [upsertLink, selectedRepoId, topicId, url, setUrl, title])
}
