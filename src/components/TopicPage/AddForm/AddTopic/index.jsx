// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import type { Relay, RepositoryType, TopicType, UserType } from 'components/types'
import upsertTopicMutation from 'mutations/upsertTopicMutation'

/* eslint jsx-a11y/label-has-for: 0 */

type Props = {
  disabled?: boolean,
  relay: Relay,
  topic: TopicType,
  viewer: UserType,
}

type State = {
  name: string,
}

class AddTopic extends Component<Props, State> {
  static defaultProps = {
    disabled: true,
  }

  state = {
    name: '',
  }

  onKeyPress = (event: Object) => {
    if (event.key === 'Enter')
      this.createTopic()
  }

  get selectedRepo(): ?RepositoryType {
    return this.props.viewer.selectedRepository
  }

  get orgLogin(): ?string {
    const repo = this.selectedRepo
    if (!repo)
      return null
    return repo.organization.login
  }

  get relayConfigs() {
    return [{
      type: 'RANGE_ADD',
      parentID: this.props.topic.id,
      connectionInfo: [{
        key: 'Topic_childTopics',
        rangeBehavior: 'prepend',
      }],
      edgeName: 'topicEdge',
    }]
  }

  updateName = (event: Object) => {
    this.setState({ name: event.currentTarget.value })
  }

  createTopic() {
    const repo = this.selectedRepo
    const repoName = repo ? repo.name : null

    upsertTopicMutation(
      this.props.relay.environment,
      this.relayConfigs,
      {
        name: this.state.name,
        repositoryName: repoName,
        organizationLogin: this.orgLogin,
        topicIds: [this.props.topic.id],
      },
    )
    this.setState({ name: '' })
  }

  render = () => (
    <dl className="form-group">
      <dt>
        <label htmlFor="create-topic-name">Add subtopic</label>
      </dt>
      <dd>
        <input
          className="form-control test-topic-name input-sm"
          disabled={this.props.disabled}
          id="create-topic-name"
          onChange={this.updateName}
          onKeyPress={this.onKeyPress}
          placeholder="Name or description"
          value={this.state.name}
        />
      </dd>
    </dl>
  )
}

export default createFragmentContainer(AddTopic, graphql`
  fragment AddTopic_viewer on User {
    selectedRepository {
      name
      organization {
        login
      }
    }
  }

  fragment AddTopic_topic on Topic {
    id
  }
`)
