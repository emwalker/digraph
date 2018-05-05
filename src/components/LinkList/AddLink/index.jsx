// @flow
import React, { Component } from 'react'
import { FormGroup, Input } from 'reactstrap'
import { graphql, createFragmentContainer } from 'react-relay'
import Select from 'react-select'
import { pathOr } from 'ramda'

import createLinkMutation from '../../../mutations/createLinkMutation'
import selectTopicMutation from '../../../mutations/selectTopicMutation'
import { liftNodes } from '../../../utils'

const selectedTopic = pathOr('', ['selectedTopic'])

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
  selectedOption: string,
  url: string,
}

class AddLink extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.options = liftNodes(props.organization.availableTopics)
    this.state = {
      selectedOption: selectedTopic(props.viewer),
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

  onSelect = (selectedOption) => {
    this.setState({ selectedOption }, () => {
      selectTopicMutation(
        this.props.relay.environment,
        {
          organizationId: this.props.organization.id,
          topicId: selectedOption ? selectedOption.value : '',
        },
      )
    })
  }

  addLink = (url) => {
    const {
      id: orgId,
      resourceId: organizationId,
    } = this.props.organization
    const topic = this.state.selectedOption
    const topicIds = topic ? [topic.value] : []

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
    <div className="form-container">
      <FormGroup>
        <Select
          id="link-topic-id"
          name="link-topic"
          onChange={this.onSelect}
          options={this.options}
          placeholder="Select a topic"
          value={this.state.selectedOption}
        />
      </FormGroup>
      <FormGroup>
        <Input
          className="link-url test-link-url"
          onChange={this.onInputChange}
          onKeyPress={this.onKeyPress}
          placeholder="Link URL"
          value={this.state.url}
        />
      </FormGroup>
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
