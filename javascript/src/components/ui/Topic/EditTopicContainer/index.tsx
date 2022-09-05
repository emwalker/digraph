import React from 'react'
import { QueryRenderer, graphql, RelayProp } from 'react-relay'

import makeEditTopic from './EditTopic'

type Props = {
  isOpen: boolean,
  relay: RelayProp,
  toggleForm: () => void,
  topicId: string,
}

const EditTopicContainer = ({ isOpen, topicId, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
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
            details {
              ...EditTopicForm_topicDetail
            }
          }
        }
      }
    `}
    variables={{
      repoName: null,
      repoIds: [],
      topicId,
      viewerId: '',
    }}
    render={makeEditTopic({ isOpen, toggleForm })}
  />
)

export default EditTopicContainer
