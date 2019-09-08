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

export default ({ viewer, defaultOrganization }: Props) => {
  const classes = classNames(header, 'clearfix d-flex px-sm-3 px-md-6 px-lg-4')

  return (
    <div className={classes}>
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
}
