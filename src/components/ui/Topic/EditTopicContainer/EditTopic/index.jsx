// @flow
import React from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Relay } from 'components/types'
import EditTopicForm from './EditTopicForm'

type Topic = {
  +id: string,
}

type View = {
  topic: Topic,
}

type Props = {
  isOpen: boolean,
  orgLogin: string,
  toggleForm: Function,
}

type PropsType = {
  error: ?Object,
  orgLogin: string,
  relay: Relay,
  topic: Topic,
  view: View,
} & Props

type RenderProps = {
  error: ?Object,
  props: ?PropsType,
}

/* eslint react/prop-types: 0 */
/* eslint react/no-unused-prop-types: 0 */

const EditTopic = (props: PropsType) => {
  const { error, topic } = props

  if (error) return <div>{error.message}</div>

  return <EditTopicForm {...props} topic={topic} />
}

const Wrapped = createFragmentContainer(EditTopic, {
  topic: graphql`
    fragment EditTopic_topic on Topic {
      id
      ...EditTopicForm_topic
    }
  `,
})

export default ({ isOpen, orgLogin, toggleForm }: Props) => ({ error, props }: RenderProps) => (
  props && props.view && props.view.topic && (
    <Wrapped
      {...props}
      error={error}
      isOpen={isOpen}
      orgLogin={orgLogin}
      relay={props.relay}
      topic={props.view.topic}
      toggleForm={toggleForm}
    />
  )
)
