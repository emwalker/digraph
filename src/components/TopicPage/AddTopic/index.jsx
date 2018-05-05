// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import createTopicMutation from '../../../mutations/createTopicMutation'
import type { RelayProps } from '../../types'

/* eslint jsx-a11y/label-has-for: 0 */

type State = {
  name: string,
}

class AddTopic extends Component<RelayProps, State> {
  state = {
    name: '',
  }

  onKeyPress = (event: Object) => {
    if (event.key === 'Enter')
      this.createTopic()
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
    const { resourceId: organizationId } = this.props.organization

    createTopicMutation(
      this.props.relay.environment,
      this.relayConfigs,
      {
        organizationId,
        name: this.state.name,
        topicIds: [this.props.topic.resourceId],
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
  fragment AddTopic_organization on Organization {
    resourceId
  }

  fragment AddTopic_topic on Topic {
    id
    resourceId
  }
`)
