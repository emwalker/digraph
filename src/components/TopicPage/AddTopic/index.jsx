// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import upsertTopicMutation from 'mutations/upsertTopicMutation'
import type { RelayProps } from '../../types'

/* eslint jsx-a11y/label-has-for: 0 */

type State = {
  name: string,
}

type Props = RelayProps & {
  viewer: {
    id: ID,
    selectedRepository: {
      name: string,
      organization: {
        login: string,
      },
    },
  },
}

class AddTopic extends Component<Props, State> {
  state = {
    name: '',
  }

  onKeyPress = (event: Object) => {
    if (event.key === 'Enter')
      this.createTopic()
  }

  get selectedRepo(): Object {
    return this.props.viewer.selectedRepository
  }

  get orgLogin(): string {
    return this.selectedRepo.organization.login
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
    upsertTopicMutation(
      this.props.relay.environment,
      this.relayConfigs,
      {
        name: this.state.name,
        repositoryName: this.selectedRepo.name,
        organizationLogin: this.orgLogin,
        topicIds: [this.props.topic.id],
      },
    )
    this.setState({ name: '' })
  }

  render = () => (
    <div>
      <dl className="form-group">
        <dt>
          <label htmlFor="create-topic-name">Add subtopic</label>
        </dt>
        <dd>
          <input
            className="form-control test-topic-name input-sm"
            id="create-topic-name"
            onChange={this.updateName}
            onKeyPress={this.onKeyPress}
            placeholder="Name or description"
            value={this.state.name}
          />
        </dd>
      </dl>
    </div>
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
