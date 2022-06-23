import React from 'react'
import { QueryRenderer, graphql, RelayProp } from 'react-relay'

import makeEditTopic from './EditTopic'

type Props = {
  isOpen: boolean,
  orgLogin: string,
  relay: RelayProp,
  toggleForm: () => void,
  topicPath: string,
}

const EditTopicContainer = ({ isOpen, orgLogin, topicPath, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditTopicContainerQuery(
        $viewerId: ID!,
        $orgLogin: String!,
        $repoName: String,
        $repoIds: [ID!],
        $topicPath: String!,
      ) {
        view(
          viewerId: $viewerId,
          currentOrganizationLogin: $orgLogin,
          currentRepositoryName: $repoName,
          repositoryIds: $repoIds,
        ) {
          topic(path: $topicPath) {
            ...EditTopicForm_topic
          }
        }
      }
    `}
    variables={{
      orgLogin,
      repoName: null,
      repoIds: [],
      topicPath,
      viewerId: '',
    }}
    render={makeEditTopic({ isOpen, orgLogin, toggleForm })}
  />
)

export default EditTopicContainer
