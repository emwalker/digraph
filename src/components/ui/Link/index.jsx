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
    title: string,
    url: string,
  },
}

type State = {
  formIsOpen: boolean,
}

class Link extends Component<Props, State> {
  state = {
    formIsOpen: false,
  }

  get parentTopics() {
    return liftNodes(this.props.link.parentTopics)
  }

  toggleForm = () => {
    this.setState(({ formIsOpen }) => ({ formIsOpen: !formIsOpen }))
  }

  render() {
    return (
      <Item
        className="Box-row--link"
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
  fragment Link_organization on Organization {
    id
  }

  fragment Link_link on Link {
    id
    title
    url

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
