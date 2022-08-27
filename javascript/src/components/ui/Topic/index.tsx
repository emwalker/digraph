import React, { Component } from 'react'
import { graphql, createFragmentContainer, RelayProp } from 'react-relay'

import { topicPath } from 'components/helpers'
import { NodeTypeOf, liftNodes } from 'components/types'
import { Topic_topic as TopicType } from '__generated__/Topic_topic.graphql'
import Item from '../Item'
import EditTopic from './EditTopicContainer'

type ParentTopicType = NodeTypeOf<TopicType['parentTopics']>

type Props = {
  relay: RelayProp,
  topic: TopicType,
}

type State = {
  formIsOpen: boolean,
}

class Topic extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      formIsOpen: false,
    }
  }

  get topicBelongsToCurrentRepo(): boolean {
    return true
  }

  get parentTopics() {
    return liftNodes<ParentTopicType>(this.props.topic.parentTopics)
  }

  get showEditButton(): boolean {
    return !this.props.topic.loading && this.props.topic.viewerCanUpdate
  }

  toggleForm = () => {
    this.setState(({ formIsOpen }) => ({ formIsOpen: !formIsOpen }))
  }

  render = () => {
    const { topic } = this.props

    return (
      <Item
        canEdit={this.props.topic.viewerCanUpdate}
        className="topicTopicRow Box-row--topic"
        description={this.props.topic.description}
        displayColor={this.props.topic.displayColor as string}
        formIsOpen={this.state.formIsOpen}
        newlyAdded={this.props.topic.newlyAdded}
        showEditButton={this.showEditButton}
        showLink={false}
        title={topic.displayName}
        toggleForm={this.toggleForm}
        topics={this.parentTopics}
        url={topicPath(topic.id)}
      >
        <EditTopic
          isOpen={this.state.formIsOpen}
          relay={this.props.relay}
          toggleForm={this.toggleForm}
          topicId={topic.id}
        />
      </Item>
    )
  }
}

export default createFragmentContainer(Topic, {
  topic: graphql`
    fragment Topic_topic on Topic {
      description
      displayName: name
      id
      loading
      newlyAdded
      viewerCanUpdate
      displayColor

      parentTopics(first: 100) {
        edges {
          node {
            id
            displayName: name
          }
        }
      }
    }
  `,
})
