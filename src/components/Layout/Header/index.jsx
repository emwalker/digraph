// @flow
import React from 'react'
import MediaQuery from 'react-responsive'

import DesktopNav from './DesktopNav'
import MobileNav from './MobileNav'

type Props = {
  viewer: Object,
  defaultOrganization: string,
}

export default ({ viewer, defaultOrganization }: Props) => (
  <div>
    <MediaQuery query="(max-width: 544px)">
      <MobileNav viewer={viewer} />
    </MediaQuery>
    <MediaQuery query="(min-width: 544px)">
      <DesktopNav
        className="clearfix mb-3 d-flex px-3 px-md-6 px-lg-4 py-2"
        viewer={viewer}
        defaultOrganization={defaultOrganization}
      />
    </MediaQuery>
  </div>
)
