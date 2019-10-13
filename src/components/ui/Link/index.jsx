// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import type { Relay } from 'components/types'
import { liftNodes } from 'utils'
import EditLink from './EditLinkContainer'
import Item from '../Item'
import type { Link_link as LinkType } from './__generated__/Link_link.graphql'
import type { Link_view as View } from './__generated__/Link_view.graphql'
import type { Link_viewer as Viewer } from './__generated__/Link_viewer.graphql'

type Props = {
  link: LinkType,
  orgLogin: string,
  relay: Relay,
  view: View,
  viewer: Viewer,
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

  get repo(): ?Object {
    return this.props.link.repository
  }

  get currentRepo(): Object {
    return this.props.view.currentRepository
  }

  get linkBelongsToCurrentRepo(): boolean {
    if (!this.repo) return true
    return this.repo.id === this.currentRepo.id
  }

  get parentTopics() {
    return liftNodes(this.props.link.parentTopics)
  }

  get displayColor(): string {
    if (!this.repo) return 'transparent'

    return this.linkBelongsToCurrentRepo
      ? 'transparent'
      : this.repo.displayColor
  }

  get showEditButton(): boolean {
    return !this.props.link.loading && !this.props.viewer.isGuest
  }

  toggleForm = () => {
    this.setState(({ formIsOpen }) => ({ formIsOpen: !formIsOpen }))
  }

  render = () => (
    <Item
      canEdit={!this.props.viewer.isGuest}
      className="Box-row--link"
      displayColor={this.displayColor}
      formIsOpen={this.state.formIsOpen}
      newlyAdded={this.props.link.newlyAdded}
      orgLogin={this.props.orgLogin}
      repoName={this.currentRepo.name}
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
      title
      url

      repository {
        displayColor
        id
      }

      parentTopics(first: 1000) {
        edges {
          node {
            displayName: name
            resourcePath
          }
        }
      }
    }
  `,
})
