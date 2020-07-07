// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Link } from 'found'
import classNames from 'classnames'

import type { Location, Router } from 'components/types'
import { toEverything } from 'components/navigation'
import DigraphLogo from 'components/ui/icons/DigraphLogo'
import SearchBox from 'components/ui/SearchBox'
import UserNav from './UserNav'
import SignIn from './SignIn'
import { header, primaryLogo, userNav, navLeft, searchBox } from './styles.module.css'
import type { DesktopNav_viewer as Viewer } from './__generated__/DesktopNav_viewer.graphql'

type Props = {
  className?: ?string,
  location: Location,
  router: Router,
  viewer: Viewer,
}

class DesktopNav extends Component<Props> {
  static defaultProps = {
    className: '',
  }

  get className(): string {
    return classNames('Header', this.props.className)
  }

  get isGuest(): boolean {
    const { viewer } = this.props

    return viewer ? viewer.isGuest : true
  }

  renderGuestUserNav = () => (
    <>
      <SignIn />
    </>
  )

  renderUserNav = (viewer: Viewer) => <UserNav viewer={viewer} />

  render = () => {
    const { viewer, router, location } = this.props

    return (
      <header className={classNames(header, 'px-sm-3 px-md-6 px-lg-4')}>
        <nav className={classNames(navLeft, 'd-inline-block')}>
          <h1 className="h3 text-normal">
            <a
              href="/"
              className={classNames(primaryLogo, 'text-gray-dark n-link no-underline d-flex')}
            >
              <div className="mr-2 d-inline-block">
                <DigraphLogo height="28px" width="28px" />
              </div>

              Digraph
            </a>
          </h1>
        </nav>
        <div className={searchBox}>
          <SearchBox router={router} location={location} />
        </div>
        <nav className={userNav}>
          <a
            className="text-gray-dark px-2"
            href="https://blog.digraph.app"
          >
            Blog
          </a>
          <Link
            className="text-gray-dark px-2"
            id="recent-activity"
            to="/recent"
          >
            Recent
          </Link>
          <Link
            className="text-gray-dark px-2"
            to={toEverything}
          >
            Everything
          </Link>
          {this.isGuest
            ? this.renderGuestUserNav()
            : this.renderUserNav(viewer)}
        </nav>
      </header>
    )
  }
}

export const UnwrappedDesktopNav = DesktopNav

export default createFragmentContainer(DesktopNav, {
  viewer: graphql`
    fragment DesktopNav_viewer on User {
      isGuest
      ...UserNav_viewer
    }
  `,
})
