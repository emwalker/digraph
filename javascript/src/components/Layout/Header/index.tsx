import React from 'react'
import { Router } from 'found'

import { LocationType } from 'components/types'
import { LayoutQueryResponse as Response } from '__generated__/LayoutQuery.graphql'
import DesktopNav from './DesktopNav'

type ViewType = Response['view']
type ViewerType = ViewType['viewer']

type Props = {
  location: LocationType,
  router: Router,
  view: ViewType,
  viewer: ViewerType,
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
