import React, {
  FormEvent, useState, useCallback,
} from 'react'
import { graphql, useFragment } from 'react-relay'

import { makeUpsertLinkCallback } from 'mutations/upsertLinkMutation'
import {
  AddLink_viewer$key,
} from '__generated__/AddLink_viewer.graphql'
import {
  AddLink_topic$key,
} from '__generated__/AddLink_topic.graphql'

const tooltip = 'Add a link to this topic.\n'
  + 'Press "Return" to submit the new link.'

type Props = {
  disabled?: boolean,
  topic: AddLink_topic$key,
  viewer: AddLink_viewer$key,
}

const viewerFragment = graphql`
  fragment AddLink_viewer on User {
    selectedRepository {
      id
    }
  }
`

const topicFragment = graphql`
  fragment AddLink_topic on Topic {
    id
  }
`

export default function AddLink(props: Props) {
  const viewer = useFragment(viewerFragment, props.viewer)
  const topic = useFragment(topicFragment, props.topic)
  const [url, setUrl] = useState('')
  const selectedRepoId = viewer.selectedRepository?.id  || null
  const onKeyPress = makeUpsertLinkCallback({ url, setUrl, topicId: topic.id, selectedRepoId })

  const onChange = useCallback((event: FormEvent<HTMLInputElement>) => {
    setUrl(event.currentTarget.value)
  }, [setUrl])

  return (
    <dl className="form-group">
      <dt>
        <span
          className="tooltipped tooltipped-ne"
          aria-label={tooltip}
        >
          <label htmlFor="create-link-url">Add link</label>
        </span>
      </dt>
      <dd>
        <input
          className="form-control input-sm"
          disabled={props.disabled}
          data-testid="link-url-input"
          id="create-link-url"
          onChange={onChange}
          onKeyPress={onKeyPress}
          placeholder="Url"
          type="url"
          value={url}
        />
      </dd>
    </dl>
  )
}
