// @flow
import React from 'react'
import MediaQuery from 'react-responsive'
import classNames from 'classnames'

import DesktopNav from './DesktopNav'
import MobileNav from './MobileNav'
import { header } from './styles.module.css'

type Props = {
  viewer: Object,
  defaultOrganization: Object,
}

export default ({ viewer, defaultOrganization }: Props) => {
  const classes = classNames(header, 'clearfix mb-3 d-flex px-md-6 px-lg-4')

  return (
    <div className={classes}>
      <div style={{ width: '100%' }}>
        <MediaQuery maxWidth={544}>
          <MobileNav viewer={viewer} />
        </MediaQuery>
        <MediaQuery minWidth={544}>
          <DesktopNav
            viewer={viewer}
            defaultOrganization={defaultOrganization}
          />
        </MediaQuery>
      </div>
    </div>
  )
}
