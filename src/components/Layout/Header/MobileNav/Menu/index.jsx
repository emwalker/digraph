// @flow
import React, { Component, Fragment } from 'react'
import { Link } from 'found'
import classNames from 'classnames'

import { toEverything } from 'components/navigation'
import type { UserType } from 'components/types'
import styles from './styles.module.css'

type Props = {
  viewer: UserType,
}

class Menu extends Component<Props> {
  renderSignIn = () => (
    <Link
      className="menu-item p-3 text-gray-dark"
      to="/login"
    >
      Sign in
    </Link>
  )

  renderUserNav = () => (
    <Fragment>
      <Link className="menu-item text-gray-dark p-3" to="/review">
        Review
      </Link>
      <Link className="menu-item text-gray-dark p-3" to="/settings/account">
        Settings
      </Link>
      <a className="menu-item text-gray-dark p-3" href="/logout">Sign out</a>
    </Fragment>
  )

  render = () => (
    <nav className={classNames(styles.menu, 'menu')} aria-label="Person settings">
      <a
        className="menu-item text-gray-dark p-3"
        href="https://blog.digraph.app"
      >
        Blog
      </a>
      <Link
        className="menu-item text-gray-dark p-3"
        id="recent-activity"
        to="/recent"
      >
        Recent
      </Link>
      <Link
        className="menu-item text-gray-dark p-3"
        to={toEverything}
      >
        Everything
      </Link>

      { this.props.viewer.isGuest
        ? this.renderSignIn()
        : this.renderUserNav()
      }
    </nav>
  )
}

export default Menu
