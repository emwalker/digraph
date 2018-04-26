// @flow
import React, { Component } from 'react'
import { Button, FormGroup, Input } from 'reactstrap'

import createTopicMutation from '../../../mutations/createTopicMutation'
import './index.scss'

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
      resourceId: organizationResourceId,
    } = this.props.organization
    createTopicMutation(
      this.props.relay.environment,
      orgId,
      {
        organizationResourceId,
        name: this.state.name,
      },
    )
    this.setState({ name: '' })
  }

  updateName = (event: Object) => {
    this.setState({ name: event.currentTarget.value })
  }

  render = () => (
    <div stylename="container">
      <FormGroup>
        <Input
          className="topic-name test-topic-name"
          placeholder="Add a topic"
          value={this.state.name}
          onChange={this.updateName}
        />
        <Button size="sm" onClick={this.onSubmit}>Submit</Button>
      </FormGroup>
    </div>
  )
}

export default AddTopic
