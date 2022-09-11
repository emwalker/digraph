import React, {
  Dispatch, SetStateAction, KeyboardEvent, FormEvent, useState, useCallback,
} from 'react'
import { graphql, useFragment, useRelayEnvironment } from 'react-relay'

import upsertLinkMutation, { Input } from 'mutations/upsertLinkMutation'
import {
  AddLink_viewer$key,
  AddLink_viewer$data as ViewerType,
} from '__generated__/AddLink_viewer.graphql'
import {
  AddLink_topic$key,
  AddLink_topic$data as TopicType,
} from '__generated__/AddLink_topic.graphql'

const tooltip = 'Add a link to this topic.\n'
  + 'Press "Return" to submit the new link.'

type Props = {
  disabled?: boolean,
  topic: AddLink_topic$key,
  viewer: AddLink_viewer$key,
}

type SetUrlType = Dispatch<SetStateAction<string>>

function upsertLink(viewer: ViewerType, setUrl: SetUrlType, url: string, topic: TopicType) {
  const repoId = viewer.selectedRepository?.id

  if (!repoId) return

  const input: Input = {
    addParentTopicId: topic.id,
    repoId,
    url,
  }

  upsertLinkMutation(useRelayEnvironment(), input)
  setUrl('')
}

export default function AddLink(props: Props) {
  const viewer = useFragment(
    graphql`
      fragment AddLink_viewer on User {
        selectedRepository {
          id
        }
      }
    `,
    props.viewer,
  )

  const topic = useFragment(
    graphql`
      fragment AddLink_topic on Topic {
        id
      }
    `,
    props.topic,
  )

  const [url, setUrl] = useState('')

  const onKeyPress = useCallback((event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') upsertLink(viewer, setUrl, url, topic)
  }, [upsertLink])

  const updateUrl = useCallback((event: FormEvent<HTMLInputElement>) => {
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
          className="form-control test-link-url input-sm"
          disabled={props.disabled}
          id="create-link-url"
          onChange={updateUrl}
          onKeyPress={onKeyPress}
          placeholder="Url"
          type="url"
          value={url}
        />
      </dd>
    </dl>
  )
}
