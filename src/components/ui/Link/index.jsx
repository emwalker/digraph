// @flow
import React, { Component } from 'react'
import { graphql, createFragmentContainer } from 'react-relay'

import { liftNodes } from '../../../utils'
import EditLink from './EditLink'
import Item from '../Item'

/* eslint no-underscore-dangle: 0 */

type Props = {
  link: {
    title: string,
    url: string,
    topics: Object,
  },
}

type State = {
  formIsOpen: boolean,
}

class Link extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.topics = liftNodes(props.link.topics)
    this.state = {
      formIsOpen: false,
    }
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
        topics={this.topics}
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
    resourceId
    ...EditLink_organization
  }

  fragment Link_link on Link {
    title
    url
    resourceId
    ...EditLink_link

    topics(first: 10) {
      edges {
        node {
          name
          resourcePath
        }
      }
    }
  }
`)
