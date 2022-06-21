import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Link, Router } from 'found'

import { LocationType } from 'components/types'
import { toEverything } from 'components/navigation'
import DigraphLogo from 'components/ui/icons/DigraphLogo'
import SearchBox from 'components/ui/SearchBox'
import { DesktopNav_viewer as Viewer } from '__generated__/DesktopNav_viewer.graphql'
import UserNav from './UserNav'
import SignIn from './SignIn'
import styles from './styles.module.css'

type Props = {
  className?: string | undefined,
  location: LocationType,
  router: Router,
  view: any,
  viewer: Viewer,
}

class DesktopNav extends Component<Props> {
  static defaultProps = {
    className: '',
  }

  get className(): string {
    return `Header ${this.props.className || ''}`
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
    const { viewer, view, router, location } = this.props

    return (
      <header className={`${styles.innerHeader} px-sm-3 px-md-6 px-lg-4`}>
        <nav className={`${styles.navLeft} d-inline-block`}>
          <h1 className="h3 text-normal">
            <a
              href="/"
              className={`${styles.primaryLogo} n-link no-underline d-flex`}
            >
              <div className="mr-2 d-inline-block">
                <DigraphLogo height="28px" width="28px" fill="#000" />
              </div>

              Digraph
            </a>
          </h1>
        </nav>
        <div className={styles.searchBox}>
          <SearchBox router={router} location={location} view={view} />
        </div>
        <nav className={styles.userNav}>
          <a
            className="Link--primary px-2"
            href="https://blog.digraph.app"
          >
            Blog
          </a>
          <Link
            className="Link--primary px-2"
            id="recent-activity"
            to="/recent"
          >
            Recent
          </Link>
          <Link
            className="Link--primary px-2"
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
