// @flow
import React from 'react'
import type { Node } from 'react'
import { Link } from 'found'

type Props = {
  children?: Node,
}

/* eslint jsx-a11y/anchor-is-valid: 0 */

const Layout = ({ children }: Props) => (
  <div>
    <div className="container">
      <div className="pagehead">
        <h1>Digraph</h1>
      </div>
      <nav className="UnderlineNav mb-3">
        <div className="UnderlineNav-body">
          <Link to="/topics" className="UnderlineNav-item" activeClassName="selected">
            Topics
          </Link>
          <Link to="/links" className="UnderlineNav-item" activeClassName="selected">
            Links
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

Layout.defaultProps = {
  children: null,
}

export default Layout
