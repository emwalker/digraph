// @flow
import React, { Component, type Node } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { pathOr } from 'ramda'
import { Link } from 'found'

import type { ViewType } from 'components/types'
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
  orgLogin: string,
  router: {
    push: Function,
  },
  view: ViewType,
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

    const to = {
      pathname: this.props.headingLink,
      orgLogin: this.props.orgLogin,
      repoName: 'Loading ...',
    }

    return (
      <Link
        className="link-gray-dark"
        to={to}
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
    <div className="Subhead clearfix gutter">
      <div className="Subhead-heading col-lg-8 col-12">
        { this.heading }
      </div>
      <SearchBox
        className="col-lg-4 col-12"
        onEnter={this.onSearch}
        value={this.searchString}
      />
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
