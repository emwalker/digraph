// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'
import Select from 'react-select'
import { path } from 'ramda'

import createLinkMutation from '../../../mutations/createLinkMutation'
import selectTopicMutation from '../../../mutations/selectTopicMutation'
import { liftNodes } from '../../../utils'

const selectedTopic = path(['selectedTopic', 'value'])

type Props = {
  organization: {
    id: string,
    resourceId: string,
    availableTopics: Array<{
      label: string,
      value: string,
    }>,
  },
  viewer: {
    selectedTopic: {
      label: string,
      value: string,
    },
  },
  relay: {
    environment: Object,
  },
}

type State = {
  selectedValue: string,
  url: string,
}

class AddLink extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.options = liftNodes(props.organization.availableTopics)
    this.state = {
      selectedValue: selectedTopic(props.viewer),
      url: '',
    }
  }

  onInputChange = (event: Object) => {
    this.setState({ url: event.currentTarget.value })
  }

  onKeyPress = (event: Object) => {
    if (event.key === 'Enter')
      this.setState(({ url }) => {
        this.addLink(url)
        return { url: '' }
      })
  }

  onSelect = (option) => {
    const selectedValue = option ? option.value : null
    this.setState({ selectedValue }, () => {
      selectTopicMutation(
        this.props.relay.environment,
        {
          organizationId: this.props.organization.id,
          topicId: selectedValue || '',
        },
      )
    })
  }

  addLink = (url) => {
    const {
      id: orgId,
      resourceId: organizationId,
    } = this.props.organization
    const value = this.state.selectedValue
    const topicIds = value ? [value] : []

    createLinkMutation(
      this.props.relay.environment,
      orgId,
      {
        organizationId,
        url,
        topicIds,
      },
    )
  }

  options: Object[]

  render = () => (
    <div>
      <dl className="form-group">
        <Select
          id="link-topic-id"
          name="link-topic"
          onChange={this.onSelect}
          options={this.options}
          placeholder="Select a topic"
          value={this.state.selectedValue}
        />
      </dl>
      <dl className="form-group">
        <input
          className="form-control test-link-url"
          onChange={this.onInputChange}
          onKeyPress={this.onKeyPress}
          placeholder="Link URL"
          value={this.state.url}
        />
      </dl>
    </div>
  )
}

export default createFragmentContainer(AddLink, graphql`
  fragment AddLink_viewer on User {
    selectedTopic {
      label: name
      value: resourceId
    }
  }

  fragment AddLink_organization on Organization {
    id
    resourceId

    availableTopics: topics(first: 100) {
      edges {
        node {
          label: name
          value: resourceId
        }
      }
    }
  }
`)
