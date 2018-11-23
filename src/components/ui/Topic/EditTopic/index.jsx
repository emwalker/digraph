// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import Input from '../../Input'
import SaveOrCancel from '../../SaveOrCancel'
import updateTopicMutation from '../../../../mutations/updateTopicMutation'

type Props = {
  id: string,
  isOpen: boolean,
  relay: {
    environment: Object,
  },
  topic: {
    description: string,
    id: string,
    name: string,
  },
  organization: {
    id: string,
  },
  toggleForm: Function,
}

type State = {
  description: string,
  name: string,
}

class EditTopic extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      name: props.topic.name,
      description: props.topic.description,
    }
  }

  onSave = () => {
    updateTopicMutation(
      this.props.relay.environment,
      [],
      {
        topicIds: this.addTopicIds,
        description: this.state.description || '',
        id: this.props.topic.id,
        name: this.state.name,
        organizationId: this.props.organization.id,
      },
    )
    this.props.toggleForm()
  }

  // eslint-disable-next-line class-methods-use-this
  get addTopicIds(): string[] {
    return []
  }

  updateDescription = (event: Object) => {
    this.setState({ description: event.currentTarget.value })
  }

  updateName = (event: Object) => {
    this.setState({ name: event.currentTarget.value })
  }

  render() {
    if (!this.props.isOpen)
      return null

    return (
      <div>
        <div className="d-flex col-12">
          <Input
            className="col-5 mr-3"
            id={`edit-link-title-${this.props.id}`}
            label="Name"
            onChange={this.updateName}
            value={this.state.name}
          />
          <Input
            className="col-6"
            id={`edit-topic-description-${this.props.id}`}
            label="Description"
            onChange={this.updateDescription}
            value={this.state.description}
          />
        </div>
        <SaveOrCancel
          onSave={this.onSave}
          onCancel={this.props.toggleForm}
        />
      </div>
    )
  }
}

export default createFragmentContainer(EditTopic, graphql`
  fragment EditTopic_organization on Organization {
    id
  }

  fragment EditTopic_topic on Topic {
    description
    id
    name
  }
`)
