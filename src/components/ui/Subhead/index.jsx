// @flow
import React, { Component } from 'react'

import { everythingTopicPath } from 'components/constants'
import SearchBox from 'components/ui/SearchBox'

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
}

class Subhead extends Component<Props> {
  onSearch = (query) => {
    if (query === '') {
      this.props.router.push({ pathname: everythingTopicPath })
      return
    }

    this.props.router.push({ pathname: everythingTopicPath, query: { q: query } })
  }

  get searchString(): string {
    return this.props.location.search
      ? this.props.location.query.q
      : ''
  }

  render = () => (
    <div className="Subhead">
      <div className="Subhead-heading">{this.props.heading}</div>
      <SearchBox onEnter={this.onSearch} value={this.searchString} />
    </div>
  )
}

export default Subhead
