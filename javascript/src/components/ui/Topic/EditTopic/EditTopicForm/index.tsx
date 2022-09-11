import React from 'react'
import { graphql, useFragment } from 'react-relay'

import EditRepoTopic from './EditRepoTopic'
import { EditTopicForm_topic$key } from '__generated__/EditTopicForm_topic.graphql'
import { EditTopicForm_viewer$key } from '__generated__/EditTopicForm_viewer.graphql'

type Props = {
  isOpen: boolean,
  toggleForm: () => void,
  topic: EditTopicForm_topic$key,
  viewer: EditTopicForm_viewer$key,
}

const topicFragment = graphql`
  fragment EditTopicForm_topic on Topic {
    displayName

    repoTopics {
      ...EditRepoTopic_repoTopic
    }
  }
`

const viewerFragment = graphql`
  fragment EditTopicForm_viewer on User {
    ...EditRepoTopic_viewer
  }
`

export default function EditTopicForm(props: Props) {
  const topic = useFragment(topicFragment, props.topic)
  const viewer = useFragment(viewerFragment, props.viewer)

  if (!props.isOpen) return null

  return (
    <div>
      <h1>Topic!</h1>

      {topic.repoTopics.map((repoTopic) => (
        <EditRepoTopic viewer={viewer} repoTopic={repoTopic} />
      ))}

      <dl className="form-group">
        <button
          className="btn-link float-right"
          onClick={props.toggleForm}
          type="button"
        >
          Close
        </button>
      </dl>
    </div>
  )
}
