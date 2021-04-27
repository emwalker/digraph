import React from 'react'
import { Link, Match } from 'found'
import classNames from 'classnames'

type Props = {
  match: Match,
}

type MenuItem = {
  pathname: string,
  label: string,
}

const menuItems = [
  { pathname: '/settings/account', label: 'Account' },
  { pathname: '/settings/support', label: 'Support' },
]

const menuItem = (match: Match, item: MenuItem) => {
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
