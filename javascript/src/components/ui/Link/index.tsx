import React, { Component } from 'react'
import { graphql, createFragmentContainer, RelayProp } from 'react-relay'

import { NodeTypeOf, liftNodes } from 'components/types'
import { Link_link as LinkType } from '__generated__/Link_link.graphql'
import { Link_viewer as ViewerType } from '__generated__/Link_viewer.graphql'
import Item from '../Item'
import EditLink from './EditLinkContainer'

type ParentTopicType = NodeTypeOf<LinkType['displayParentTopics']>

type Props = {
  link: LinkType,
  relay: RelayProp,
  viewer: ViewerType,
}

type State = {
  formIsOpen: boolean,
}

class Link extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      formIsOpen: false,
    }
  }

  get linkBelongsToCurrentRepo(): boolean {
    return true
  }

  get parentTopics() {
    return liftNodes<ParentTopicType>(this.props.link.displayParentTopics)
  }

  get showEditButton(): boolean {
    return !this.props.link.loading && this.props.link.viewerCanUpdate
  }

  toggleForm = () => {
    this.setState(({ formIsOpen }) => ({ formIsOpen: !formIsOpen }))
  }

  render = () => (
    <Item
      canEdit={!this.props.viewer.isGuest}
      className="Box-row--link"
      displayColor={this.props.link.displayColor as string}
      formIsOpen={this.state.formIsOpen}
      newlyAdded={this.props.link.newlyAdded}
      showEditButton={this.showEditButton}
      showLink
      title={this.props.link.displayTitle}
      toggleForm={this.toggleForm}
      topics={this.parentTopics}
      url={this.props.link.displayUrl}
    >
      <EditLink
        isOpen={this.state.formIsOpen}
        link={this.props.link}
        relay={this.props.relay}
        toggleForm={this.toggleForm}
      />
    </Item>
  )
}

export const UnwrappedLink = Link

export default createFragmentContainer(Link, {
  viewer: graphql`
    fragment Link_viewer on User {
      isGuest
    }
  `,
  link: graphql`
    fragment Link_link on Link {
      displayColor
      displayTitle
      displayUrl
      id
      loading
      newlyAdded
      viewerCanUpdate

      displayParentTopics(first: 100) {
        edges {
          node {
            displayName
            id
          }
        }
      }
    }
  `,
})
