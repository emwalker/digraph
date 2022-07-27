import React, { Component } from 'react'
import { Link } from 'found'
import { createFragmentContainer, graphql } from 'react-relay'

import { toEverything } from 'components/navigation'
import { Menu_viewer as Viewer } from '__generated__/Menu_viewer.graphql'

type Props = {
  viewer: Viewer,
}

class Menu extends Component<Props> {
  renderSignIn = () => (
    <Link
      className="menu-item p-3 Link--primary"
      to="/login"
    >
      Sign in
    </Link>
  )

  renderUserNav = () => (
    <>
      <Link className="menu-item Link--primary p-3" to="/review">
        Review
      </Link>
      <Link className="menu-item Link--primary p-3" to="/settings/account">
        Settings
      </Link>
      <a className="menu-item Link--primary p-3" href="/logout">Sign out</a>
    </>
  )

  render = () => (
    <nav className="mobile-menu menu" aria-label="Person settings">
      <a
        className="menu-item Link--primary p-3"
        href="https://blog.digraph.app"
      >
        Blog
      </a>
      <Link
        className="menu-item Link--primary p-3"
        id="recent-activity"
        to="/recent"
      >
        Recent
      </Link>
      <Link
        className="menu-item Link--primary p-3"
        to={toEverything}
      >
        Everything
      </Link>

      { this.props.viewer.isGuest
        ? this.renderSignIn()
        : this.renderUserNav()}
    </nav>
  )
}

export default createFragmentContainer(Menu, {
  viewer: graphql`
    fragment Menu_viewer on User {
      isGuest
    }
  `,
})
