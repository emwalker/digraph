// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { TopicType, OrganizationType } from '../../../types'
import Input from '../../Input'
import SaveOrCancel from '../../SaveOrCancel'
import updateTopicMutation from '../../../../mutations/updateTopicMutation'
import updateTopicTopicsMutation from '../../../../mutations/updateTopicParentTopicsMutation'
import EditTopicList from '../../EditTopicList'
import { liftNodes } from '../../../../utils'

type Props = {
  id: string,
  isOpen: boolean,
  organization: OrganizationType,
  relay: {
    environment: Object,
  },
  toggleForm: Function,
  topic: TopicType,
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

  get availableTopics(): Object[] {
    return liftNodes(this.props.topic.availableTopics)
  }

  get selectedTopics(): string[] {
    return liftNodes(this.props.topic.selectedTopics)
  }

  updateParentTopics = (parentTopicIds: string[]) => {
    updateTopicTopicsMutation(
      this.props.relay.environment,
      [],
      {
        topicId: this.props.topic.id,
        parentTopicIds,
      },
    )
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
        <EditTopicList
          availableTopics={this.availableTopics}
          selectedTopics={this.selectedTopics}
          updateTopics={this.updateParentTopics}
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

    selectedTopics: parentTopics(first: 1000) {
      edges {
        node {
          id
          name
        }
      }
    }

    availableTopics: availableParentTopics(first: 1000) {
      edges {
        node {
          id
          name
        }
      }
    }
  }
`)
