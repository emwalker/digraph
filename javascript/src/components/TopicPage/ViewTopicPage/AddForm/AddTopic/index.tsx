import React, { FormEvent, MouseEvent, useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import { topicPath } from 'components/helpers'
import { makeUpsertTopic, MatchingTopicsType } from 'mutations/upsertTopicMutation'
import { AddTopic_viewer$key } from '__generated__/AddTopic_viewer.graphql'
import { AddTopic_parentTopic$key } from '__generated__/AddTopic_parentTopic.graphql'
import { AlertMessageType } from 'components/types'
import Alert from 'components/FlashMessages/Alert'

const tooltipText = 'Add a subtopic to this topic. You can click "Edit"\n'
  + 'afterwards if it also belongs under another topic.\n'
  + 'Press "Return" to submit the new topic.'

type Props = {
  disabled?: boolean,
  parentTopic: AddTopic_parentTopic$key,
  viewer: AddTopic_viewer$key,
}

const topicFragment = graphql`
  fragment AddTopic_parentTopic on Topic {
    id
  }
`

const viewerFragment = graphql`
  fragment AddTopic_viewer on User {
    selectedRepoId
  }
`

function makeAlert(alert: AlertMessageType, matchingTopics: MatchingTopicsType) {
  if (!/existing topic/.test(alert.text) || matchingTopics.length === 0)
    return <Alert key={alert.id} alert={alert} />

  const removeAlert = window.flashMessages?.removeAlert
  const onClick = () => removeAlert && removeAlert(alert)

  return (
    <Alert key={alert.id} alert={alert}>
      <ul className="px-5 py-3">
        {matchingTopics.map((topic) => {
          const href = topicPath(topic.id)

          return (
            <li className="p-1">
              Add <a href={href} target="_blank">{ topic.displayName }</a> to the current parent
              topic
              <button type="button" className="btn btn-secondary btn-sm ml-2">update</button>
            </li>
          )
        })}

        <li className="p-1">
          Create a new topic
          <button type="button" className="btn btn-secondary btn-sm ml-2">create</button>
        </li>

        <li className="p-1"> <a href="#" onClick={onClick}>Do nothing</a> </li>
      </ul>
    </Alert>
  )
}

export default function AddTopic(props: Props) {
  const viewer = useFragment(viewerFragment, props.viewer)
  const parentTopic = useFragment(topicFragment, props.parentTopic)
  const [name, setName] = useState('')

  const selectedRepoId = viewer.selectedRepoId
  const onKeyPress = makeUpsertTopic({
    selectedRepoId, name, setName, topicId: parentTopic.id, makeAlert,
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
