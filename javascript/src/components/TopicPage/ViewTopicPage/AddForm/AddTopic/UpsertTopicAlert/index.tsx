import React from 'react'

import { MatchingTopicsType } from 'mutations/upsertTopicMutation'
import { AlertMessageType } from 'components/types'
import Alert from 'components/FlashMessages/Alert'
import UpdateTopic from './UpdateTopic'
import CreateTopic from './CreateTopic'

type Props = {
  alert: AlertMessageType,
  matchingTopics: MatchingTopicsType,
  name: string,
  parentTopicId: string,
  selectedRepoId: string,
}

export default function UpsertTopicAlert({
  alert, matchingTopics, selectedRepoId, name, parentTopicId,
}: Props) {
  if (!/existing topic/.test(alert.text) || matchingTopics.length === 0)
    return <Alert alert={alert} />

  const removeAlert = window.flashMessages?.removeAlert
  const removeHandler = () => removeAlert && removeAlert(alert)

  return (
    <Alert alert={alert}>
      <ul className="px-5 py-3">
        {matchingTopics.map((topic) => (
          <UpdateTopic
            key={topic.id}
            name={name}
            parentTopicId={parentTopicId}
            removeAlert={removeHandler}
            selectedRepoId={selectedRepoId}
            updateTopic={topic}
          />
        ))}

        <CreateTopic
          name={name}
          parentTopicId={parentTopicId}
          removeAlert={removeHandler}
          selectedRepoId={selectedRepoId}
        />

        <li className="p-1"> <a href="#" onClick={removeHandler}>Do nothing</a> </li>
      </ul>
    </Alert>
  )
}
