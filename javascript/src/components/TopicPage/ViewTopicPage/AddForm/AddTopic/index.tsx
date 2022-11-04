import React, { FormEvent, KeyboardEvent, useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import { AlertMessageType } from 'components/types'
import { makeUpsertTopic, MatchingTopicsType } from 'mutations/upsertTopicMutation'
import { AddTopic_viewer$key } from '__generated__/AddTopic_viewer.graphql'
import { AddTopic_parentTopic$key } from '__generated__/AddTopic_parentTopic.graphql'
import UpsertTopicAlert from './UpsertTopicAlert'

const tooltipText = 'Add a subtopic to this topic. You can click "Edit"\n'
  + 'afterwards if it also belongs under another topic.\n'
  + 'Press "Return" to submit the new topic.'

type Props = {
  disabled?: boolean,
  parentTopic: AddTopic_parentTopic$key,
  viewer: AddTopic_viewer$key,
}

const parentTopicFragment = graphql`
  fragment AddTopic_parentTopic on Topic {
    id
  }
`

const viewerFragment = graphql`
  fragment AddTopic_viewer on User {
    selectedRepoId
  }
`

export default function AddTopic(props: Props) {
  const viewer = useFragment(viewerFragment, props.viewer)
  const parentTopic = useFragment(parentTopicFragment, props.parentTopic)
  const [name, setName] = useState('')

  const selectedRepoId = viewer.selectedRepoId
  if (!selectedRepoId) return null

  const makeAlert = useCallback(
    (alert: AlertMessageType, matchingTopics: MatchingTopicsType) => (
      <UpsertTopicAlert
        alert={alert}
        matchingTopics={matchingTopics}
        name={name}
        parentTopicId={parentTopic.id}
        selectedRepoId={selectedRepoId}
      />
    ),
    [name, selectedRepoId],
  )

  const ignoreEvent = useCallback(
    (event: KeyboardEvent<HTMLInputElement>) => event.key !== 'Enter',
    [],
  )

  const onKeyPress = makeUpsertTopic({
    selectedRepoId, name, setName, parentTopicId: parentTopic.id, makeAlert, onMatchingSynonym: 'ASK',
    ignoreEvent,
  })

  const updateName = useCallback((event: FormEvent<HTMLInputElement>) => {
    setName(event.currentTarget.value)
  }, [setName])

  return (
    <dl className="form-group">
      <dt>
        <span
          className="tooltipped tooltipped-ne"
          aria-label={tooltipText}
        >
          <label htmlFor="create-topic-name">Add subtopic</label>
        </span>
      </dt>
      <dd>
        <input
          className="form-control test-topic-name input-sm"
          disabled={props.disabled}
          id="create-topic-name"
          onChange={updateName}
          onKeyPress={onKeyPress}
          placeholder="Name or description"
          data-testid="topic-name-input"
          value={name}
        />
      </dd>
    </dl>
  )
}
