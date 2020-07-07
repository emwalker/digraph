// @flow
import React from 'react'
import { MediaQuery } from 'react-responsive-hoc'
import classNames from 'classnames'

import type { Location, Router } from 'components/types'
import DesktopNav from './DesktopNav'
import MobileNav from './MobileNav'
import { header } from './styles.module.css'

type Props = {
  defaultOrganization: Object,
  location: Location,
  router: Router,
  viewer: Object,
}

export default ({ defaultOrganization, location, router, viewer }: Props) => {
  const classes = classNames(header, 'clearfix d-flex')

  return (
    <div className={classes}>
      <div style={{ width: '100%' }}>
        <MediaQuery query="(max-width: 1280px)">
          <MobileNav
            location={location}
            router={router}
            viewer={viewer}
          />
        </MediaQuery>
        <MediaQuery query="(min-width: 1280px)">
          <DesktopNav
            defaultOrganization={defaultOrganization}
            location={location}
            router={router}
            viewer={viewer}
          />
        </MediaQuery>
      </div>
    </div>
  )
}
