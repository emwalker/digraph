import React from 'react'
import { useLazyLoadQuery, graphql } from 'react-relay'

import EditTopicForm from './EditTopicForm'
import {
  EditTopicContainerQuery,
} from '__generated__/EditTopicContainerQuery.graphql'

type Props = {
  isOpen: boolean,
  toggleForm: () => void,
  topicId: string,
  viewerId: string | null,
}

export default function EditTopicContainer({ isOpen, topicId, toggleForm, viewerId }: Props) {
  const data = useLazyLoadQuery<EditTopicContainerQuery>(
    graphql`
      query EditTopicContainerQuery(
        $viewerId: ID!,
        $repoIds: [ID!],
        $topicId: String!,
      ) {
        view(
          viewerId: $viewerId,
          repositoryIds: $repoIds,
        ) {
          viewer {
            ...EditTopicForm_viewer
          }

          topic(id: $topicId) {
            ...EditTopicForm_topic
          }
        }
      }
    `,
    {
      repoIds: [],
      topicId,
      viewerId: viewerId || '',
    },
  )

  if (!data.view || !data.view.topic) return null
  const topic = data.view.topic

  return (
    <EditTopicForm
      isOpen={isOpen}
      toggleForm={toggleForm}
      // @ts-expect-error
      topic={topic}
      // @ts-expect-error
      viewer={data.view.viewer}
    />
  )
}