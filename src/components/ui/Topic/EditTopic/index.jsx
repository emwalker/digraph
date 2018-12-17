// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { TopicType } from 'components/types'
import EditTopicForm from './EditTopicForm'

type RendererProps = {
  error: ?Object,
  props: ?{
    orgLogin: string,
    view: {
      link: LinkType,
    },
    viewer: Oobject,
  },
}

/* eslint react/prop-types: 0 */
/* eslint react/no-unused-prop-types: 0 */

const renderer = ({ isOpen, orgLogin, toggleForm }) => ({ error, props }: RendererProps) => {
  if (error)
    return <div>{error.message}</div>

  if (!props || !props.view)
    return null

  return (
    <EditTopicForm
      isOpen={isOpen}
      orgLogin={orgLogin}
      topic={props.view.topic}
      viewer={props.viewer}
      toggleForm={toggleForm}
    />
  )
}

type Props = {
  isOpen: boolean,
  topic: TopicType,
  relay: {
    environment: Object,
  },
  toggleForm: Function,
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
