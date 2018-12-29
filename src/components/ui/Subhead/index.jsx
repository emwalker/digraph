// @flow
import React, { Component, type Node } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { pathOr } from 'ramda'
import { Link } from 'found'

import SearchBox from 'components/ui/SearchBox'

const resourcePath = pathOr('/', ['currentRepository', 'rootTopic', 'resourcePath'])

type Props = {
  heading: string,
  headingLink: ?string,
  location: {
    pathname: string,
    query: Object,
    search: string,
  },
  router: {
    push: Function,
  },
  view: {
    currentRepository: {
      rootTopic: {
        resourcePath: string,
      },
    },
  },
}

class Subhead extends Component<Props> {
  static defaultProps = {
    headingLink: null,
  }

  onSearch = (query) => {
    if (query === '') {
      this.props.router.push({ pathname: this.pathname })
      return
    }

    this.props.router.push({ pathname: this.pathname, query: { q: query } })
  }

  get heading(): Node {
    if (!this.props.headingLink)
      return this.props.heading

    return (
      <Link
        className="link-gray-dark"
        to={this.props.headingLink}
      >
        {this.props.heading}
      </Link>
    )
  }

  get pathname(): string {
    return resourcePath(this.props.view)
  }

  get searchString(): string {
    return this.props.location.search
      ? this.props.location.query.q
      : ''
  }

  render = () => (
    <div className="Subhead">
      <div className="Subhead-heading">
        {this.heading}
      </div>
      <SearchBox onEnter={this.onSearch} value={this.searchString} />
    </div>
  )
}

export default createFragmentContainer(Subhead, graphql`
  fragment Subhead_view on View {
    currentRepository {
      rootTopic {
        resourcePath
      }
    }
  }
`)
