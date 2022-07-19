import React, { Component } from 'react'
import { graphql, createFragmentContainer, RelayProp } from 'react-relay'

import { NodeTypeOf, liftNodes } from 'components/types'
import { Link_link as LinkType } from '__generated__/Link_link.graphql'
import { Link_view as ViewType } from '__generated__/Link_view.graphql'
import { Link_viewer as ViewerType } from '__generated__/Link_viewer.graphql'
import Item from '../Item'
import EditLink from './EditLinkContainer'

type ParentTopicType = NodeTypeOf<LinkType['parentTopics']>

type Props = {
  link: LinkType,
  orgLogin: string,
  relay: RelayProp,
  view: ViewType,
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

  get repo() {
    return this.props.link.repository
  }

  get currentRepo() {
    return this.props.view.currentRepository
  }

  get linkBelongsToCurrentRepo(): boolean {
    if (!this.repo) return true
    return this.repo.id === this.currentRepo?.id
  }

  get parentTopics() {
    return liftNodes<ParentTopicType>(this.props.link.parentTopics)
  }

  get displayColor() {
    if (!this.repo) return 'transparent'

    return this.linkBelongsToCurrentRepo
      ? 'transparent'
      : this.repo.displayColor
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
      displayColor={this.displayColor as string}
      formIsOpen={this.state.formIsOpen}
      newlyAdded={this.props.link.newlyAdded}
      orgLogin={this.props.orgLogin}
      repoName={this.currentRepo?.name || null}
      showEditButton={this.showEditButton}
      showLink
      title={this.props.link.title}
      toggleForm={this.toggleForm}
      topics={this.parentTopics}
      url={this.props.link.url}
    >
      <EditLink
        isOpen={this.state.formIsOpen}
        link={this.props.link}
        orgLogin={this.props.orgLogin}
        relay={this.props.relay}
        toggleForm={this.toggleForm}
      />
    </Item>
  )
}

export const UnwrappedLink = Link

export default createFragmentContainer(Link, {
  view: graphql`
    fragment Link_view on View {
      currentRepository {
        name
        id
      }
    }
  `,
  viewer: graphql`
    fragment Link_viewer on User {
      isGuest
    }
  `,
  link: graphql`
    fragment Link_link on Link {
      id
      loading
      newlyAdded
      path
      title
      url
      viewerCanUpdate

      repository {
        displayColor
        id
      }

      parentTopics(first: 100) {
        edges {
          node {
            displayName: name
            path
          }
        }
      }
    }
  `,
})
