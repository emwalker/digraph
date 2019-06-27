// @flow
import React from 'react'
import { MediaQuery } from 'react-responsive-hoc'
import classNames from 'classnames'

import DesktopNav from './DesktopNav'
import MobileNav from './MobileNav'
import { header } from './styles.module.css'

type Props = {
  viewer: Object,
  defaultOrganization: Object,
}

export default ({ viewer, defaultOrganization }: Props) => (
  <div className={classNames(header, 'clearfix mb-3 d-flex px-3 px-md-6 px-lg-4 py-2')}>
    <div style={{ width: '100%' }}>
      <MediaQuery query="(max-width: 544px)">
        <MobileNav viewer={viewer} />
      </MediaQuery>
      <MediaQuery query="(min-width: 544px)">
        <DesktopNav
          viewer={viewer}
          defaultOrganization={defaultOrganization}
        />
      </MediaQuery>
    </div>
  </div>
)
