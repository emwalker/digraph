import React, { FormEvent, MouseEvent, useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import { makeUpsertTopic } from 'mutations/upsertTopicMutation'
import { AddTopic_viewer$key } from '__generated__/AddTopic_viewer.graphql'
import { AddTopic_parentTopic$key } from '__generated__/AddTopic_parentTopic.graphql'
import { AlertMessageType } from 'components/types'
import Alert from 'components/FlashMessages/Alert'
import CreateTopicButton from './CreateTopicButton'
import UpdateTopicButton from './UpdateTopicButton'

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

function makeAlert(alert: AlertMessageType) {
  return (
    <Alert key={alert.id} alert={alert}>
      <div className="d-flex flex-justify-center flex-items-center">
        <div className="m-2"> <UpdateTopicButton /> </div>
        <div className="m-2"> <CreateTopicButton /> </div>
        <div className="m-2"> <a href="#">Do nothing</a> </div>
      </div>
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
