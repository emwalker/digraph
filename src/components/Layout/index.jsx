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
          Digraffe
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
  </div>
)

Layout.defaultProps = {
  children: null,
}

export default Layout
