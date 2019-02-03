// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import type { LinkType, Relay, UserType, ViewType } from 'components/types'
import { liftNodes } from 'utils'
import EditLink from './EditLinkContainer'
import Item from '../Item'

/* eslint no-underscore-dangle: 0 */

type Props = {
  link: LinkType,
  orgLogin: string,
  relay: Relay,
  view: ViewType,
  viewer: UserType,
}

type State = {
  formIsOpen: boolean,
}

class Link extends Component<Props, State> {
  state = {
    formIsOpen: false,
  }

  get repo(): ?Object {
    return this.props.link.repository
  }

  get currentRepo(): Object {
    return this.props.view.currentRepository
  }

  get linkBelongsToCurrentRepo(): boolean {
    if (!this.repo)
      return true
    return this.repo.id === this.currentRepo.id
  }

  get parentTopics() {
    return liftNodes(this.props.link.parentTopics)
  }

  get displayColor(): string {
    if (!this.repo)
      return 'transparent'

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
      className="Box-row--link"
      displayColor={this.displayColor}
      formIsOpen={this.state.formIsOpen}
      newlyAdded={this.props.link.newlyAdded}
      orgLogin={this.props.orgLogin}
      repoName={this.currentRepo.name}
      showEditButton={this.showEditButton}
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

export default createFragmentContainer(Link, graphql`
  fragment Link_view on View {
    currentRepository {
      name
      id
    }
  }

  fragment Link_viewer on User {
    isGuest
  }

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

    parentTopics(first: 10) {
      edges {
        node {
          name
          resourcePath
        }
      }
    }
  }
`)
