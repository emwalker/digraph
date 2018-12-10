// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import upsertLinkMutation from 'mutations/upsertLinkMutation'
import type { RelayProps } from '../../types'

/* eslint jsx-a11y/label-has-for: 0 */

type Props = RelayProps & {
  viewer: {
    defaultRepository: {
      id: ID,
    },
  },
}

type State = {
  url: string,
}

class AddLink extends Component<Props, State> {
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

  get repositoryId(): ?string {
    if (this.props.viewer)
      return this.props.viewer.defaultRepository.id
    return null
  }

  updateUrl = (event: Object) => {
    this.setState({ url: event.currentTarget.value })
  }

  createLink() {
    upsertLinkMutation(
      this.props.relay.environment,
      this.relayConfigs,
      {
        addParentTopicIds: [this.props.topic.id],
        repositoryId: this.repositoryId,
        url: this.state.url,
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
  fragment AddLink_topic on Topic {
    id
  }
`)
