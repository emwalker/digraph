import React, { FormEvent, useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import { makeUpsertTopic } from 'mutations/upsertTopicMutation'
import { AddTopic_viewer$key } from '__generated__/AddTopic_viewer.graphql'
import { AddTopic_topic$key } from '__generated__/AddTopic_topic.graphql'

const tooltipText = 'Add a subtopic to this topic. You can click "Edit"\n'
  + 'afterwards if it also belongs under another topic.\n'
  + 'Press "Return" to submit the new topic.'

type Props = {
  disabled?: boolean,
  topic: AddTopic_topic$key,
  viewer: AddTopic_viewer$key,
}

const topicFragment = graphql`
  fragment AddTopic_topic on Topic {
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
  const topic = useFragment(topicFragment, props.topic)
  const [name, setName] = useState('')

  const selectedRepoId = viewer.selectedRepoId
  const onKeyPress = makeUpsertTopic({ selectedRepoId, name, setName, topicId: topic.id })

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
