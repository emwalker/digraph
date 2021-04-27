import React from 'react'
import { QueryRenderer, graphql, RelayProp } from 'react-relay'

import makeEditTopic from './EditTopic'

type Props = {
  isOpen: boolean,
  orgLogin: string,
  relay: RelayProp,
  toggleForm: () => void,
  topicId: string,
}

const EditTopicContainer = ({ isOpen, orgLogin, topicId, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditTopicContainerQuery(
        $viewerId: ID!,
        $orgLogin: String!,
        $repoName: String,
        $repoIds: [ID!],
        $topicId: ID!,
      ) {
        view(
          viewerId: $viewerId,
          currentOrganizationLogin: $orgLogin,
          currentRepositoryName: $repoName,
          repositoryIds: $repoIds,
        ) {
          topic(id: $topicId) {
            ...EditTopicForm_topic
          }
        }
      }
    `}
    variables={{
      orgLogin,
      repoName: null,
      repoIds: [],
      topicId,
      viewerId: '',
    }}
    render={makeEditTopic({ isOpen, orgLogin, toggleForm })}
  />
)

export default EditTopicContainer
