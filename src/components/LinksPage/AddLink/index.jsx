// @flow
import React, { Component } from 'react'
import { FormGroup, Input } from 'reactstrap'
import { graphql, createFragmentContainer } from 'react-relay'
import Select from 'react-select'

import createLinkMutation from '../../../mutations/createLinkMutation'
import { liftNodes } from '../../../utils'

type Props = {
  organization: {
    id: string,
    resourceId: string,
    availableTopics: Array<{
      label: string,
      value: string,
    }>,
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
  }

  state = {
    selectedOption: '',
    url: '',
  }

  onInputChange = (event: Object) => {
    this.setState({ url: event.currentTarget.value })
  }

  onKeyPress = (event: Object) => {
    if (event.key === 'Enter')
      this.setState(({ url }) => {
        this.addUrl(url)
        return { url: '' }
      })
  }

  onSelect = (selectedOption) => {
    this.setState({ selectedOption })
  }

  addUrl = (url) => {
    const {
      id: orgId,
      resourceId: organizationResourceId,
    } = this.props.organization
    createLinkMutation(
      this.props.relay.environment,
      orgId,
      { organizationResourceId, url },
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
  fragment AddLink_organization on Organization {
    id
    resourceId

    availableTopics: topics(first: 100) {
      edges {
        node {
          label: name
          value: resourcePath
        }
      }
    }
  }
`)
