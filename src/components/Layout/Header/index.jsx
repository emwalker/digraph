// @flow
import React from 'react'
import { MediaQuery } from 'react-responsive-hoc'

import type { Location, Router } from 'components/types'
import DesktopNav from './DesktopNav'
import MobileNav from './MobileNav'
import { header } from './styles.module.css'

type Props = {
  defaultOrganization: Object,
  location: Location,
  router: Router,
  view: Object,
  viewer: Object,
}

export default ({ defaultOrganization, location, router, viewer, view }: Props) => (
  <div className={`${header} clearfix d-flex`}>
    <div style={{ width: '100%' }}>
      <MediaQuery query="(max-width: 768px)">
        <MobileNav
          location={location}
          router={router}
          viewer={viewer}
          showButton={false}
        />
      </MediaQuery>
      <MediaQuery query="(min-width: 769px) and (max-width: 1280px)">
        <MobileNav
          location={location}
          router={router}
          viewer={viewer}
        />
      </MediaQuery>
      <MediaQuery query="(min-width: 1281px)">
        <DesktopNav
          defaultOrganization={defaultOrganization}
          location={location}
          router={router}
          view={view}
          viewer={viewer}
        />
      </MediaQuery>
    </div>
  </div>
)
