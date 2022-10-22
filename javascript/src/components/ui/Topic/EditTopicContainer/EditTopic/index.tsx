import React from 'react'
import { graphql, useFragment } from 'react-relay'

import { backgroundColor, borderColor } from 'components/helpers'
import EditRepoTopic from './EditRepoTopic'
import ViewRepoTopic from './ViewRepoTopic'
import { EditTopic_topic$key } from '__generated__/EditTopic_topic.graphql'
import { EditTopic_viewer$key } from '__generated__/EditTopic_viewer.graphql'

type Props = {
  topic: EditTopic_topic$key,
  viewer: EditTopic_viewer$key,
}

const viewerFragment = graphql`
  fragment EditTopic_viewer on User {
    selectedRepoId
    ...EditRepoTopic_viewer
  }
`

const topicFragment = graphql`
  fragment EditTopic_topic on Topic {
    displayName

    repoTopics {
      repo {
        name
        id
      }

      viewerCanUpdate
      displayColor

      ...EditRepoTopic_repoTopic
      ...ViewRepoTopic_repoTopic
    }
  }
`

export default function EditTopic(props: Props) {
  const topic = useFragment(topicFragment, props.topic)
  const viewer = useFragment(viewerFragment, props.viewer)

  return (
    <div className="mt-3" data-testid="edit-topic">
      {topic.repoTopics.map((repoTopic, index) => {
        const { id: repoId, name: repoName } = repoTopic.repo
        const showEditForm = repoTopic.viewerCanUpdate && viewer.selectedRepoId === repoId

        return (
          <ul
            key={index}
            className="Box Box--condensed mt-3"
            data-testid={`repo-topic-${repoId}`}
            style={{ borderColor: borderColor(repoTopic.displayColor) }}
          >
            <div
              className="Box-header"
              style={{
                backgroundColor: backgroundColor(repoTopic.displayColor),
                borderColor: borderColor(repoTopic.displayColor),
              }}
            >
              {repoName}
            </div>

            {showEditForm
              ? <EditRepoTopic viewer={viewer} repoTopic={repoTopic} />
              : <ViewRepoTopic repoTopic={repoTopic} />}
          </ul>
        )
      })}
    </div>
  )
}
