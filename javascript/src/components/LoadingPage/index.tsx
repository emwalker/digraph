import React from 'react'

import { LocationType } from 'components/types'
import Page from 'components/ui/Page'
import SearchBoxPlaceholder from 'components/ui/SearchBox/Placeholder'
import Columns from 'components/ui/Columns'
import LeftColumn from 'components/ui/LeftColumn'
import RightColumn from 'components/ui/RightColumn'
import SidebarList from 'components/ui/SidebarList'

type Props = {
  location: LocationType,
}

const LoadingPage = ({ location }: Props) => {
  const state = location.state || {}
  const { itemTitle } = state

  return (
    <Page>
      <div className="Subhead clearfix gutter">
        <div className="Subhead-heading col-lg-8 col-12">
          { itemTitle }
        </div>
        <SearchBoxPlaceholder />
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
