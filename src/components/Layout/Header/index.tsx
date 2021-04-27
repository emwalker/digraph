import React from 'react'
import { Router } from 'found'
import { MediaQuery } from 'react-responsive-hoc'

import { LocationType } from 'components/types'
import { LayoutQueryResponse as Response } from '__generated__/LayoutQuery.graphql'
import DesktopNav from './DesktopNav'
import MobileNav from './MobileNav'
import { header } from './styles.module.css'

type ViewType = Response['view']
type ViewerType = ViewType['viewer']

type Props = {
  location: LocationType,
  router: Router,
  view: ViewType,
  viewer: ViewerType,
}

export default ({ location, router, viewer, view }: Props) => (
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
          location={location}
          router={router}
          view={view}
          viewer={viewer}
        />
      </MediaQuery>
    </div>
  </div>
)
