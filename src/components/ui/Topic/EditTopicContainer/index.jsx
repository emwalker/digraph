// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { Relay } from 'components/types'
import makeEditTopic from './EditTopic'
import type { EditTopic_topic as Topic } from './__generated__/EditTopicContainerQuery.graphql'

type Props = {
  isOpen: boolean,
  orgLogin: string,
  relay: Relay,
  toggleForm: Function,
  topic: Topic,
}

const EditTopicContainer = ({ isOpen, orgLogin, topic, relay, toggleForm }: Props) => (
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
            ...EditTopic_topic
          }
        }
      }
    `}
    variables={{
      orgLogin,
      repoName: null,
      repoIds: [],
      topicId: topic.id,
      viewerId: '',
    }}
    render={makeEditTopic({ isOpen, orgLogin, relay, toggleForm })}
  />
)

export default EditTopicContainer
