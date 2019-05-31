// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { pathOr } from 'ramda'
import DocumentTitle from 'react-document-title'

import type { ViewType } from 'components/types'
import SearchBox from 'components/ui/SearchBox'

const resourcePath = pathOr('/', ['currentRepository', 'rootTopic', 'resourcePath'])

type Props = {
  heading: string,
  location: {
    pathname: string,
    query: Object,
    search: string,
  },
  router: {
    push: Function,
  },
  view: ViewType,
}

class Subhead extends Component<Props> {
  onSearch = (query: string) => {
    if (query === '') {
      this.props.router.push({ pathname: this.pathname })
      return
    }

    this.props.router.push({ pathname: this.pathname, query: { q: query } })
  }

  get pathname(): string {
    return resourcePath(this.props.view)
  }

  get searchString(): string {
    return this.props.location.search
      ? this.props.location.query.q
      : ''
  }

  get title(): string {
    return `${this.props.heading} | Digraph`
  }

  render = () => (
    <DocumentTitle title={this.title}>
      <div className="Subhead clearfix gutter">
        <div className="Subhead-heading col-lg-8 col-12">
          { this.props.heading }
        </div>
        <SearchBox
          className="col-lg-4 col-12"
          onEnter={this.onSearch}
          value={this.searchString}
        />
      </div>
    </DocumentTitle>
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
