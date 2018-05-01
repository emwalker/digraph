// @flow
import React, { Component } from 'react'
import { Button, FormGroup, Input } from 'reactstrap'

import createLinkMutation from '../../../mutations/createLinkMutation'

type Props = {
  organization: {
    id: string,
    resourceId: string,
  },
  relay: {
    environment: Object,
  },
}

type State = {
  url: string,
}

class AddLink extends Component<Props, State> {
  state = {
    url: '',
  }

  onSubmit = () => {
    const {
      id: orgId,
      resourceId: organizationResourceId,
    } = this.props.organization
    createLinkMutation(
      this.props.relay.environment,
      orgId,
      {
        organizationResourceId,
        url: this.state.url,
      },
    )
    this.setState({ url: '' })
  }

  updateUrl = (event: Object) => {
    this.setState({ url: event.currentTarget.value })
  }

  render = () => (
    <div className="form-container">
      <FormGroup>
        <Input
          className="link-url test-link-url"
          placeholder="Link URL"
          value={this.state.url}
          onChange={this.updateUrl}
        />
        <Button size="sm" onClick={this.onSubmit}>Add</Button>
      </FormGroup>
    </div>
  )
}

export default AddLink
