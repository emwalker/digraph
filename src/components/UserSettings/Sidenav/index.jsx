// @flow
import React from 'react'
import { Link } from 'found'
import classNames from 'classnames'

import type { Match } from 'components/types'

type Props = {
  match: Match,
}

const menuItems = [
  { pathname: '/settings/account', label: 'Account' },
  { pathname: '/settings/support', label: 'Support' },
]

const menuItem = (match, item) => {
  const { location: { pathname } } = match

  return (
    <Link
      className={classNames('menu-item', { selected: pathname === item.pathname })}
      key={item.pathname}
      to={item.pathname}
    >
      { item.label }
    </Link>
  )
}

const Sidenav = ({ match }: Props) => (
  <nav className="menu col-3 float-left" aria-label="Settings">
    <span className="menu-heading" id="menu-heading">Settings</span>
    { menuItems.map((item) => menuItem(match, item)) }
  </nav>
)

export default Sidenav
