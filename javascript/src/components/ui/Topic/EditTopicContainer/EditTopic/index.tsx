import React from 'react'
import { graphql, useFragment } from 'react-relay'

import { backgroundColor, borderColor } from 'components/helpers'
import EditRepoTopic from './EditRepoTopic'
import ViewRepoTopic from './ViewRepoTopic'
import { EditTopic_topic$key } from '__generated__/EditTopic_topic.graphql'

type Props = {
  topic: EditTopic_topic$key,
  viewer: any,
}

const topicFragment = graphql`
  fragment EditTopic_topic on Topic {
    displayName

    repoTopics {
      repo {
        name
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

  return (
    <div className="mt-3">
      {topic.repoTopics.map((repoTopic, index) => (
        <ul
          key={index}
          className="Box Box--condensed mt-3"
          style={{ borderColor: borderColor(repoTopic.displayColor) }}
        >
          <div
            className="Box-header"
            style={{
              backgroundColor: backgroundColor(repoTopic.displayColor),
              borderColor: borderColor(repoTopic.displayColor),
            }}
          >
            {repoTopic.repo.name}
          </div>

          {repoTopic.viewerCanUpdate
            ? <EditRepoTopic viewer={props.viewer} repoTopic={repoTopic} />
            : <ViewRepoTopic repoTopic={repoTopic} />}
        </ul>
      ))}
    </div>
  )
}
