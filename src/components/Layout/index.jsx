// @flow
import React, { Component } from 'react'
import type { Node } from 'react'
import { Link } from 'found'

import FilterInput from './FilterInput'

type Props = {
  children?: Node,
  location: {
    pathname: string,
    search: string,
    query: Object,
  },
  router: {
    push: Function,
  },
}

/* eslint jsx-a11y/anchor-is-valid: 0 */

class Layout extends Component<Props> {
  static defaultProps = {
    children: null,
  }

  onSearch = (query) => {
    const { pathname } = this.props.location

    if (query === '') {
      this.props.router.push({ pathname })
      return
    }

    this.props.router.push({ pathname, query: { q: query } })
  }

  get searchString(): string {
    return this.props.location.search
      ? this.props.location.query.q
      : ''
  }

  render = () => {
    const { children } = this.props

    return (
      <div>
        <div className="container">
          <div className="pagehead">
            <div className="container-lg clearfix">
              <h1 className="float-left">Digraph</h1>
              <FilterInput onEnter={this.onSearch} value={this.searchString} />
            </div>
          </div>
          <nav className="UnderlineNav mb-3">
            <div className="UnderlineNav-body">
              <Link to="/links" className="UnderlineNav-item" activeClassName="selected">
                Links
              </Link>
              <Link to="/topics" className="UnderlineNav-item" activeClassName="selected">
                Topics
              </Link>
            </div>
          </nav>
          { children }
        </div>
        <div className="container">
          <footer className="my-6 pt-4 border-top">
            <p className="mb-2">
              Available for use under the MIT{' '}
              <a href="https://github.com/emwalker/digraph/blob/master/LICENSE.md">license</a>.
              © Eric Walker.
            </p>
          </footer>
        </div>
      </div>
    )
  }
}

export default Layout
