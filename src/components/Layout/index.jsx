// @flow
import React from 'react'
import type { Node } from 'react'
import { Link } from 'found'

type Props = {
  children?: Node,
}

const Layout = ({ children }: Props) => (
  <div>
    <header className="masthead">
      <div className="container">
        <a className="masthead-logo" href="/">
          Digraph
        </a>
        <nav className="masthead-nav">
          <Link
            className="test-topics-page"
            activeClassName="active"
            href="/topics"
            to="/topics"
          >
            Topics
          </Link>
          <Link
            className="test-links-page"
            activeClassName="active"
            href="/links"
            to="/links"
          >
            Links
          </Link>
        </nav>
      </div>
    </header>
    <div className="container">
      { children }
    </div>
    <div className="container">
      <footer className="my-6 pt-4 border-top">
        <p className="mb-2">
          Available for use under the MIT{' '}
          <a href="https://github.com/emwalker/digraph/blob/master/LICENSE.md">license</a>.
          Copyright Eric Walker 2018.
          Derived from Github’s <a href="https://primer.github.io/">Primer</a>.
        </p>
      </footer>
    </div>
  </div>
)

Layout.defaultProps = {
  children: null,
}

export default Layout
