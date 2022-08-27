import React from 'react'
import { Router } from 'found'

import { LocationType } from 'components/types'
import Page from 'components/ui/Page'
import SearchBox from 'components/ui/SearchBox'
import Columns from 'components/ui/Columns'
import LeftColumn from 'components/ui/LeftColumn'
import RightColumn from 'components/ui/RightColumn'
import SidebarList from 'components/ui/SidebarList'

type Props = {
  location: LocationType,
  router: Router,
}

const LoadingPage = ({ location, router }: Props) => {
  const state = location.state || {}
  const { itemTitle } = state

  return (
    <Page>
      <div className="Subhead clearfix gutter">
        <div className="Subhead-heading col-lg-8 col-12">
          { itemTitle }
        </div>
        <SearchBox
          className="col-lg-4 col-12"
          location={location}
          router={router}
          value=""
        />
      </div>
      <div>
        <Columns>
          <RightColumn>
            <SidebarList
              title="Parent topics"
              placeholder="There are no parent topics for this topic."
              items={[]}
            />
          </RightColumn>
          <LeftColumn>
            <div className="blankslate">
              <p>Searching the computers for items ...</p>
            </div>
          </LeftColumn>
        </Columns>
      </div>
    </Page>
  )
}

export default LoadingPage
