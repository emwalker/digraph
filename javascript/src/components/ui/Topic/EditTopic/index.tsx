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
}

export default function EditTopicContainer({ isOpen, topicId, toggleForm }: Props) {
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
          topic(id: $topicId) {
            ...EditTopicForm_topic
          }
        }
      }
    `,
    {
      repoIds: [],
      topicId,
      viewerId: '',
    },
  )

  if (!data.view || !data.view.topic) return null
  const topic = data.view.topic

  return (
    <EditTopicForm
      isOpen={isOpen}
      toggleForm={toggleForm}
      // @ts-ignore-next-line
      topic={topic}
    />
  )
}