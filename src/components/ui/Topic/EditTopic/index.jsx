// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { TopicType } from 'components/types'
import { defaultOrganizationId } from 'components/constants'
import EditTopicForm from './EditTopicForm'

type RendererProps = {
  error: ?Object,
  props: ?{
    view: {
      link: LinkType,
    },
  },
}

/* eslint react/prop-types: 0 */
/* eslint react/no-unused-prop-types: 0 */

const renderer = ({ isOpen, toggleForm }) => ({ error, props }: RendererProps) => {
  if (error)
    return <div>{error.message}</div>

  if (!props || !props.view)
    return null

  return (
    <EditTopicForm
      isOpen={isOpen}
      topic={props.view.topic}
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

const EditTopic = ({ isOpen, topic, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditTopicQuery($orgIds: [ID!], $topicId: ID!) {
        view(organizationIds: $orgIds) {
          topic(id: $topicId) {
            ...EditTopicForm_topic
          }
        }
      }
    `}
    variables={{
      topicId: topic.id,
      orgIds: [defaultOrganizationId],
    }}
    render={renderer({ isOpen, toggleForm })}
  />
)

export default EditTopic
