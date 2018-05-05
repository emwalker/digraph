// @flow
import React, { Component } from 'react'

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
    <div>
      <dl className="form-group">
        <dd>
          <input
            className="form-control test-topic-name"
            placeholder="Topic"
            value={this.state.name}
            onChange={this.updateName}
          />
        </dd>
        <button
          size="sm"
          className="d-inline-block btn btn-secondary mt-2"
          onClick={this.onSubmit}
        >
          Add
        </button>
      </dl>
    </div>
  )
}

export default AddTopic
