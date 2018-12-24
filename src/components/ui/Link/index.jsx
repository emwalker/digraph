// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import { liftNodes } from '../../../utils'
import EditLink from './EditLink'
import Item from '../Item'

/* eslint no-underscore-dangle: 0 */

type Props = {
  link: {
    parentTopics: Object,
    repository: {
      id: string,
    },
    title: string,
    url: string,
  },
  view: {
    currentRepository: {
      id: string,
    },
  },
}

type State = {
  formIsOpen: boolean,
}

class Link extends Component<Props, State> {
  state = {
    formIsOpen: false,
  }

  get repo(): Object {
    return this.props.link.repository
  }

  get linkBelongsToCurrentRepository(): boolean {
    return this.repo.id === this.props.view.currentRepository.id
  }

  get parentTopics() {
    return liftNodes(this.props.link.parentTopics)
  }

  get displayColor(): string {
    return this.linkBelongsToCurrentRepository
      ? 'transparent'
      : this.repo.displayColor
  }

  toggleForm = () => {
    this.setState(({ formIsOpen }) => ({ formIsOpen: !formIsOpen }))
  }

  render() {
    return (
      <Item
        className="Box-row--link"
        displayColor={this.displayColor}
        formIsOpen={this.state.formIsOpen}
        title={this.props.link.title}
        toggleForm={this.toggleForm}
        topics={this.parentTopics}
        url={this.props.link.url}
      >
        <EditLink
          link={this.props.link}
          toggleForm={this.toggleForm}
          isOpen={this.state.formIsOpen}
          {...this.props}
        />
      </Item>
    )
  }
}

export default createFragmentContainer(Link, graphql`
  fragment Link_view on View {
    currentRepository {
      id
    }
  }

  fragment Link_link on Link {
    id
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
