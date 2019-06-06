// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Relay, TopicType, UserType, ViewType } from 'components/types'
import { liftNodes } from 'utils'
import EditTopicForm from './EditTopicForm'

type Props = {
  isOpen: boolean,
  orgLogin: string,
  toggleForm: Function,
}

type PropsType = {
  error: ?Object,
  orgLogin: string,
  relay: Relay,
  topic: TopicType,
  view: ViewType,
  viewer: UserType,
} & Props

type RenderProps = {
  error: ?Object,
  props: ?PropsType,
}

/* eslint react/prop-types: 0 */
/* eslint react/no-unused-prop-types: 0 */

const EditTopic = (props: PropsType) => {
  const { error, isOpen, orgLogin, relay, toggleForm, topic, viewer } = props

  if (error) return <div>{error.message}</div>

  if (!topic) return null

  return (
    <EditTopicForm
      availableTopics={liftNodes(topic.availableTopics)}
      isOpen={isOpen}
      orgLogin={orgLogin}
      relay={relay}
      selectedTopics={liftNodes(topic.selectedTopics)}
      toggleForm={toggleForm}
      topic={topic}
      viewer={viewer}
    />
  )
}

const Wrapped = createFragmentContainer(EditTopic, {
  topic: graphql`
    fragment EditTopic_topic on Topic {
      ...EditTopicForm_topic
    }
  `,
})

export default ({ isOpen, orgLogin, toggleForm }: Props) => ({ error, props }: RenderProps) => {
  if (!props) return null

  const { view, viewer } = props

  return (
    <Wrapped
      error={error}
      isOpen={isOpen}
      orgLogin={orgLogin}
      relay={props.relay}
      toggleForm={toggleForm}
      topic={view.topic}
      view={view}
      viewer={viewer}
    />
  )
}
