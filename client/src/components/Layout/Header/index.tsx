import React from 'react'
import { Router } from 'found'

import { LocationType } from 'components/types'
import DesktopNav from './DesktopNav'

type Props = {
  location: LocationType,
  router: Router,
  view: any,
  viewer: any,
}

export default ({ location, router, viewer, view }: Props) => (
  <div className="outerHeader clearfix d-flex">
    <DesktopNav
      location={location}
      router={router}
      view={view}
      viewer={viewer}
    />
  </div>
)
