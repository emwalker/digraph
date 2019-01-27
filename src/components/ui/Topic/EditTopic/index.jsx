// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { Relay, TopicType, ViewType, UserType } from 'components/types'
import { liftNodes } from 'utils'
import EditTopicForm from './EditTopicForm'

type RendererProps = {
  error: ?Object,
  props: ?{
    orgLogin: string,
    relay: Relay,
    view: ViewType,
    viewer: UserType,
  },
}

/* eslint react/prop-types: 0 */
/* eslint react/no-unused-prop-types: 0 */

const renderer = ({ isOpen, orgLogin, toggleForm }) => ({ error, props }: RendererProps) => {
  if (error)
    return <div>{error.message}</div>

  if (!props || !props.view || !props.view.topic)
    return null

  const { view: { topic } } = props

  return (
    <EditTopicForm
      availableTopics={liftNodes(topic.availableTopics)}
      isOpen={isOpen}
      orgLogin={orgLogin}
      relay={props.relay}
      selectedTopics={liftNodes(topic.selectedTopics)}
      toggleForm={toggleForm}
      topic={topic}
      viewer={props.viewer}
    />
  )
}

type Props = {
  isOpen: boolean,
  orgLogin: string,
  relay: {
    environment: Object,
  },
  toggleForm: Function,
  topic: TopicType,
  view: Object,
}

const EditTopic = ({ isOpen, orgLogin, topic, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditTopicQuery(
        $orgLogin: String!,
        $repoName: String,
        $repoIds: [ID!],
        $topicId: ID!,
      ) {
        viewer {
          ...EditTopicForm_viewer
        }

        view(
          currentOrganizationLogin: $orgLogin,
          currentRepositoryName: $repoName,
          repositoryIds: $repoIds,
        ) {
          topic(id: $topicId) {
            selectedTopics: parentTopics(first: 1000) {
              edges {
                node {
                  id
                  name
                }
              }
            }

            availableTopics: availableParentTopics(first: 10) {
              edges {
                node {
                  id
                  name
                }
              }
            }

            ...EditTopicForm_topic
          }
        }
      }
    `}
    variables={{
      orgLogin,
      repoName: null,
      repoIds: [],
      topicId: topic.id,
    }}
    render={renderer({ isOpen, orgLogin, toggleForm })}
  />
)

export default EditTopic
