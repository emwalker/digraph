// @flow
import React, { Component } from 'react'
import { createRefetchContainer, graphql } from 'react-relay'

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

class EditTopic extends Component<PropsType> {
  componentDidMount = () => {
    const { refetch } = this.props.relay
    if (refetch) {
      setTimeout(() => {
        refetch({
          orgLogin: this.props.orgLogin,
          count: 1000,
        })
      }, 100)
    }
  }

  render = () => {
    const { error, isOpen, orgLogin, relay, toggleForm, topic, viewer } = this.props

    if (error)
      return <div>{error.message}</div>

    if (!topic)
      return null

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
}

const Wrapped = createRefetchContainer(EditTopic, graphql`
  fragment EditTopic_topic on Topic @argumentDefinitions(
    count: {type: "Int!", defaultValue: 10}
  ) {
    selectedTopics: parentTopics(first: 3000) {
      edges {
        node {
          id
          name
        }
      }
    }

    availableTopics: availableParentTopics(first: $count) {
      edges {
        node {
          id
          name
        }
      }
    }

    ...EditTopicForm_topic
  }
`, graphql`
  query EditTopicRefetchQuery(
    $orgLogin: String!,
    $repoName: String,
    $repoIds: [ID!],
    $topicId: ID!,
    $count: Int!,
  ) {
    view(
      currentOrganizationLogin: $orgLogin,
      currentRepositoryName: $repoName,
      repositoryIds: $repoIds,
    ) {
      topic(id: $topicId) {
        ...EditTopic_topic @arguments(count: $count)
      }
    }
  }
`)

export default ({ isOpen, orgLogin, toggleForm }: Props) => ({ error, props }: RenderProps) => {
  if (!props)
    return null

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
