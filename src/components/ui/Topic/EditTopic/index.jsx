// @flow
import React from 'react'
import { QueryRenderer, graphql } from 'react-relay'

import type { TopicType, OrganizationType } from 'components/types'
import EditTopicForm from './EditTopicForm'

type RendererProps = {
  error: ?Object,
  props: ?{
    organization: {
      link: LinkType,
    },
  },
}

/* eslint react/prop-types: 0 */

const renderer = ({ isOpen, toggleForm }) => ({ error, props }: RendererProps) => {
  if (error)
    return <div>{error.message}</div>

  if (!props || !props.organization)
    return null

  return (
    <EditTopicForm
      isOpen={isOpen}
      topic={props.organization.topic}
      organization={props.organization}
      toggleForm={toggleForm}
    />
  )
}

type Props = {
  isOpen: boolean,
  topic: TopicType,
  organization: OrganizationType,
  relay: {
    environment: Object,
  },
  toggleForm: Function,
}

const EditTopic = ({ isOpen, topic, organization, relay, toggleForm }: Props) => (
  <QueryRenderer
    environment={relay.environment}
    query={graphql`
      query EditTopicQuery($organizationId: ID!, $topicId: ID!) {
        organization(id: $organizationId) {
          ...EditTopicForm_organization

          topic(id: $topicId) {
            ...EditTopicForm_topic
          }
        }
      }
    `}
    variables={{
      topicId: topic.id,
      organizationId: organization.id,
    }}
    render={renderer({ isOpen, toggleForm })}
  />
)

export default EditTopic
