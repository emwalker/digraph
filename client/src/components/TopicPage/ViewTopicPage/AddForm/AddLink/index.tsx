import React, {
  FormEvent, useState, useCallback, KeyboardEvent,
} from 'react'
import { graphql, useFragment } from 'react-relay'

import { makeUpsertLinkCallback } from 'mutations/upsertLinkMutation'
import {
  AddLink_viewer$key,
} from '__generated__/AddLink_viewer.graphql'
import {
  AddLink_parentTopic$key,
} from '__generated__/AddLink_parentTopic.graphql'

const tooltip = 'Add a link to this topic.\n'
  + 'Press "Return" to submit the new link.'

type Props = {
  disabled?: boolean,
  parentTopic: AddLink_parentTopic$key,
  viewer: AddLink_viewer$key,
}

const viewerFragment = graphql`
  fragment AddLink_viewer on User {
    selectedRepoId
  }
`

const topicFragment = graphql`
  fragment AddLink_parentTopic on Topic {
    id
  }
`

export default function AddLink(props: Props) {
  const viewer = useFragment(viewerFragment, props.viewer)
  const parentTopic = useFragment(topicFragment, props.parentTopic)
  const [url, setUrl] = useState('')
  const selectedRepoId = viewer.selectedRepoId
  const upsertLink = makeUpsertLinkCallback({
    url, setUrl, topicId: parentTopic.id, selectedRepoId,
  })

  const onKeyPress = useCallback((event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key !== 'Enter') return
    upsertLink()
  }, [upsertLink])

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
