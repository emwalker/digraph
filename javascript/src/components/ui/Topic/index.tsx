import React, { Component } from 'react'
import { graphql, createFragmentContainer, RelayProp } from 'react-relay'
import classNames from 'classnames'

import { NodeTypeOf, liftNodes } from 'components/types'
import { Topic_topic as TopicType } from '__generated__/Topic_topic.graphql'
import { Topic_view as ViewType } from '__generated__/Topic_view.graphql'
import Item from '../Item'
import EditTopic from './EditTopicContainer'
import styles from './styles.module.css'

type ParentTopicType = NodeTypeOf<TopicType['parentTopics']>

type Props = {
  orgLogin: string,
  relay: RelayProp,
  topic: TopicType,
  view: ViewType,
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

  get repo() {
    return this.props.topic.repository
  }

  get currentRepo() {
    return this.props.view.currentRepository
  }

  get topicBelongsToCurrentRepo(): boolean {
    if (!this.repo) return true
    return this.repo.id === this.currentRepo?.id
  }

  get displayColor() {
    return this.topicBelongsToCurrentRepo
      ? 'transparent'
      : this.props.topic.repository.displayColor as string
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
        className={classNames(styles.topicRow, 'Box-row--topic')}
        description={this.props.topic.description}
        displayColor={this.displayColor}
        formIsOpen={this.state.formIsOpen}
        newlyAdded={this.props.topic.newlyAdded}
        orgLogin={this.props.orgLogin}
        repoName={topic.repository && topic.repository.name}
        showEditButton={this.showEditButton}
        showLink={false}
        title={topic.displayName}
        toggleForm={this.toggleForm}
        topics={this.parentTopics}
        url={topic.path}
      >
        <EditTopic
          isOpen={this.state.formIsOpen}
          orgLogin={this.props.orgLogin}
          relay={this.props.relay}
          toggleForm={this.toggleForm}
          topicPath={topic.path}
        />
      </Item>
    )
  }
}

export default createFragmentContainer(Topic, {
  view: graphql`
    fragment Topic_view on View {
      currentRepository {
        id
      }
    }
  `,
  topic: graphql`
    fragment Topic_topic on Topic {
      description
      displayName: name
      id
      loading
      newlyAdded
      path
      viewerCanUpdate

      repository {
        name
        displayColor
        id
      }

      parentTopics(first: 100) {
        edges {
          node {
            id
            displayName: name
            path
          }
        }
      }
    }
  `,
})
