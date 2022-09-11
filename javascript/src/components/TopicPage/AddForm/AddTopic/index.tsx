import React, {
  SetStateAction, KeyboardEvent, FormEvent, useCallback, useState, Dispatch,
} from 'react'
import {
  graphql, DeclarativeMutationConfig, useRelayEnvironment, useFragment,
} from 'react-relay'

import upsertTopicMutation from 'mutations/upsertTopicMutation'
import {
  AddTopic_viewer$key,
  AddTopic_viewer$data as ViewerType,
} from '__generated__/AddTopic_viewer.graphql'
import {
  AddTopic_topic$key,
  AddTopic_topic$data as TopicType,
} from '__generated__/AddTopic_topic.graphql'

type RepositoryType = ViewerType['selectedRepository']

const tooltipText = 'Add a subtopic to this topic. You can click "Edit"\n'
  + 'afterwards if it also belongs under another topic.\n'
  + 'Press "Return" to submit the new topic.'

type Props = {
  disabled?: boolean,
  topic: AddTopic_topic$key,
  viewer: AddTopic_viewer$key,
}

type SetNameType = Dispatch<SetStateAction<string>>;

function relayConfigs(parentID: string): DeclarativeMutationConfig[] {
  return [{
    type: 'RANGE_ADD',
    parentID,
    connectionInfo: [{
      key: 'Topic_children',
      rangeBehavior: 'prepend',
    }],
    edgeName: 'topicEdge',
  }]
}

function createTopic(repo: RepositoryType, topic: TopicType, name: string, setName: SetNameType) {
  if (!repo) {
    // eslint-disable-next-line no-console
    console.log('missing repo')
    return
  }

  const repoId = repo.id
  if (!repoId) {
    console.log('expected a repo id', repo)
    return
  }

  upsertTopicMutation(
    useRelayEnvironment(),
    {
      name,
      repoId,
      parentTopicId: topic.id,
    },
    {
      configs: relayConfigs(topic.id),
    },
  )

  setName('')
}

export default function AddTopic(props: Props) {
  const viewer = useFragment(
    graphql`
      fragment AddTopic_viewer on User {
        selectedRepository {
          id
        }
      }
    `,
    props.viewer,
  )

  const topic = useFragment(
    graphql`
      fragment AddTopic_topic on Topic {
        id
      }
    `,
    props.topic,
  )

  const [name, setName] = useState('')
  const selectedRepo = viewer.selectedRepository

  const onKeyPress = useCallback((event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key !== 'Enter') return
    createTopic(selectedRepo, topic, name, setName)
  }, [createTopic, selectedRepo, topic, name, setName])

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
          value={name}
        />
      </dd>
    </dl>
  )
}
