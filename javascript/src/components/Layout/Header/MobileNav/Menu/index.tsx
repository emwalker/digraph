import React from 'react'
import { Link } from 'found'
import classNames from 'classnames'
import { graphql, useFragment } from 'react-relay'

import { toEverything } from 'components/navigation'
import { Menu_viewer$key  } from '__generated__/Menu_viewer.graphql'
import styles from './styles.module.css'

type Props = {
  viewer: Menu_viewer$key,
}

const renderSignIn = () => (
  <Link
    className="menu-item p-3 Link--primary"
    to="/login"
  >
    Sign in
  </Link>
)

const renderUserNav = () => (
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

export default function Menu(props: Props) {
  const viewer = useFragment(
    graphql`
      fragment Menu_viewer on User {
        isGuest
      }
    `,
    props.viewer,
  )

  return (
    <nav className={classNames(styles.menu, 'menu')} aria-label="Person settings">
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

      {viewer.isGuest
        ? renderSignIn()
        : renderUserNav()}
    </nav>
  )
}
