// @flow
import React from 'react'
import type { Node } from 'react'

type Props = {
  children?: Node,
}

const Layout = ({ children }: Props) => (
  <div>
    <nav className="navbar navbar-expand-lg navbar-light bg-light flex-column flex-md-row bd-navbar">
      <a className="navbar-brand mb-0 h1" href="/">Digraffe</a>
      <ul className="navbar-nav mr-auto">
        <li className="nav-item">
          <a className="nav-link" href="/topics">Topics</a>
        </li>
      </ul>
    </nav>
    <div className="container">
      { children }
    </div>
  </div>
)

Layout.defaultProps = {
  children: null,
}

export default Layout
