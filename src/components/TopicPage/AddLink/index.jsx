// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import createLinkMutation from '../../../mutations/createLinkMutation'
import type { RelayProps } from '../../types'

/* eslint jsx-a11y/label-has-for: 0 */

type State = {
  url: string,
}

class AddLink extends Component<RelayProps, State> {
  state = {
    url: '',
  }

  onKeyPress = (event: Object) => {
    if (event.key === 'Enter')
      this.createLink()
  }

  get relayConfigs() {
    return [{
      type: 'RANGE_ADD',
      parentID: this.props.topic.id,
      connectionInfo: [{
        key: 'Topic_links',
        rangeBehavior: 'prepend',
      }],
      edgeName: 'linkEdge',
    }]
  }

  updateUrl = (event: Object) => {
    this.setState({ url: event.currentTarget.value })
  }

  createLink() {
    const { resourceId: organizationId } = this.props.organization

    createLinkMutation(
      this.props.relay.environment,
      this.relayConfigs,
      {
        organizationId,
        url: this.state.url,
        topicIds: [this.props.topic.resourceId],
      },
    )
    this.setState({ url: '' })
  }

  render = () => (
    <div>
      <dl className="form-group">
        <dt>
          <label htmlFor="create-link-url">Add link</label>
        </dt>
        <dd>
          <input
            className="form-control test-link-url input-sm"
            id="create-link-url"
            onChange={this.updateUrl}
            onKeyPress={this.onKeyPress}
            placeholder="Url"
            value={this.state.url}
          />
        </dd>
      </dl>
    </div>
  )
}

export default createFragmentContainer(AddLink, graphql`
  fragment AddLink_organization on Organization {
    resourceId
  }

  fragment AddLink_topic on Topic {
    id
    resourceId
  }
`)
