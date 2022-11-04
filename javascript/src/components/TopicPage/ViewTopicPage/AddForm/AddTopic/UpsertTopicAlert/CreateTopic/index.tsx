import React, { useCallback, MouseEvent } from 'react'

import { AlertMessageType } from 'components/types'
import { makeUpsertTopic, MatchingTopicsType } from 'mutations/upsertTopicMutation'
import UpsertTopicAlert from '..'

type Props = {
  name: string,
  parentTopicId: string,
  removeAlert: () => void,
  selectedRepoId: string,
}

export default function CreateTopic({ selectedRepoId, name, parentTopicId, removeAlert }: Props) {
  const makeAlert = useCallback((alert: AlertMessageType, matchingTopics: MatchingTopicsType) => (
    <UpsertTopicAlert
      alert={alert}
      matchingTopics={matchingTopics}
      name={name}
      parentTopicId={parentTopicId}
      selectedRepoId={selectedRepoId}
    />
  ), [name, selectedRepoId])

  const createTopic = makeUpsertTopic({
    selectedRepoId, name, parentTopicId, makeAlert, onMatchingSynonym: 'CREATE_DISTINCT',
  })

  const onClick = useCallback((event: MouseEvent<HTMLButtonElement>) => {
    createTopic(event)
    removeAlert()
  }, [createTopic, removeAlert])

  return (
    <li className="p-1">
      Create a new topic
      <button type="button" className="btn btn-secondary btn-sm ml-2" onClick={onClick}>
        create
      </button>
    </li>
  )
}
