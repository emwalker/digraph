// @flow
import React, { Component } from 'react'
import { Button, FormGroup, Input } from 'reactstrap'

import createTopicMutation from '../../../mutations/createTopicMutation'

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
  name: string,
}

class AddTopic extends Component<Props, State> {
  state = {
    name: '',
  }

  onSubmit = () => {
    const {
      id: orgId,
      resourceId: organizationId,
    } = this.props.organization
    createTopicMutation(
      this.props.relay.environment,
      orgId,
      {
        organizationId,
        name: this.state.name,
      },
    )
    this.setState({ name: '' })
  }

  updateName = (event: Object) => {
    this.setState({ name: event.currentTarget.value })
  }

  render = () => (
    <div className="form-container">
      <FormGroup>
        <Input
          className="topic-name test-topic-name"
          placeholder="Topic name"
          value={this.state.name}
          onChange={this.updateName}
        />
        <Button size="sm" onClick={this.onSubmit}>Add</Button>
      </FormGroup>
    </div>
  )
}

export default AddTopic
