// @flow
import React from 'react'

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
        <ul className="user-nav d-lg-flex flex-items-center list-style-none">
          {viewer && (
            <li className="dropdown">
              <details
                className="details-overlay details-reset d-none d-lg-flex pl-lg-2 py-2 py-lg-0 flex-items-center"
              >
                <summary className="HeaderNavlink name mt-1">
                  { viewer.name }
                  <span className="dropdown-caret" />
                </summary>
              </details>
            </li>
          )}
        </ul>
      </div>
    </div>
  </header>
)

export default Header
