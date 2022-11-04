import React, { useCallback, MouseEvent } from 'react'
import { graphql, useFragment } from 'react-relay'

import { AlertMessageType } from 'components/types'
import { topicPath } from 'components/helpers'
import { makeUpsertTopic, MatchingTopicsType } from 'mutations/upsertTopicMutation'
import { UpdateTopic_updateTopic$key } from '__generated__/UpdateTopic_updateTopic.graphql'
import UpsertTopicAlert from '..'

type Props = {
  name: string,
  parentTopicId: string,
  removeAlert: () => void,
  selectedRepoId: string,
  updateTopic: UpdateTopic_updateTopic$key,
}

const updateTopicFragment = graphql`
  fragment UpdateTopic_updateTopic on Topic {
    id
    displayName
  }
`

export default function UpdateTopic({
  name, selectedRepoId, parentTopicId, removeAlert, ...rest
}: Props) {
  const topicToUpdate = useFragment(updateTopicFragment, rest.updateTopic)
  const href = topicPath(topicToUpdate.id)

  const makeAlert = useCallback((alert: AlertMessageType, matchingTopics: MatchingTopicsType) => (
    <UpsertTopicAlert
      alert={alert}
      matchingTopics={matchingTopics}
      name={name}
      selectedRepoId={selectedRepoId}
      parentTopicId={parentTopicId}
    />
  ), [name, selectedRepoId])

  const updateTopic = makeUpsertTopic({
    selectedRepoId, name, parentTopicId, makeAlert, updateTopicId: topicToUpdate.id,
    onMatchingSynonym: 'UPDATE',
  })

  const onClick = useCallback((event: MouseEvent<HTMLButtonElement>) => {
    updateTopic(event)
    removeAlert()
  }, [updateTopic, removeAlert])

  return (
    <li className="p-1">
      Add <a href={href} target="_blank">{topicToUpdate.displayName}</a> to the current parent
      topic
      <button type="button" className="btn btn-secondary btn-sm ml-2" onClick={onClick}>
        update
      </button>
    </li>
  )
}
