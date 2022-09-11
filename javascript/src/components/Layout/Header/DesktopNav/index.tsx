import React from 'react'
import { graphql, useFragment } from 'react-relay'
import { Link, Router } from 'found'

import { LocationType } from 'components/types'
import { toEverything } from 'components/navigation'
import DigraphLogo from 'components/ui/icons/DigraphLogo'
import SearchBox from 'components/ui/SearchBox'
import { DesktopNav_viewer$key } from '__generated__/DesktopNav_viewer.graphql'
import UserNav from './UserNav'
import SignIn from './SignIn'

type Props = {
  location: LocationType,
  router: Router,
  view: any,
  viewer: DesktopNav_viewer$key,
}

export default function DesktopNav(props: Props) {
  const viewer = useFragment(
    graphql`
      fragment DesktopNav_viewer on User {
        isGuest
        ...UserDropdown_viewer
      }
    `,
    props.viewer,
  )

  const isGuest = viewer.isGuest === undefined ? true : viewer.isGuest

  return (
    <header className="innerHeader px-sm-3 px-md-6 px-lg-4">
      <nav className="navLeft d-inline-block">
        <h1 className="h3 text-normal">
          <a
            href="/"
            className="primaryLogo n-link no-underline d-flex"
          >
            <div className="mr-2 d-inline-block">
              <DigraphLogo height="28px" width="28px" fill="#000" />
            </div>

            Digraph
          </a>
        </h1>
      </nav>
      <div className="searchBox">
        <SearchBox router={props.router} location={props.location} view={props.view} />
      </div>
      <nav className="userNav">
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
        {isGuest
          ? <SignIn />
          : <UserNav viewer={viewer} />}
      </nav>
    </header>
  )
}
