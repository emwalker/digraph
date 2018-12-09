// @flow
import React from 'react'

import ViewerDropdown from './ViewerDropdown'
import GithubLogin from './GithubLogin'

type Props = {
    viewer: ?Object,
}

const Header = ({ viewer }: Props) => (
  <header className="Header pagehead">
    <div className="container-lg clearfix">
      <nav className="d-lg-flex float-left">
        <h1>Digraph</h1>
      </nav>
      <div className="d-lg-flex float-right mt-1">
        <ul className="user-nav d-lg-flex list-style-none">
          <li className="dropdown">
            {viewer
              ? <ViewerDropdown viewer={viewer} />
              : <GithubLogin />
            }
          </li>
        </ul>
      </div>
    </div>
  </header>
)

export default Header
