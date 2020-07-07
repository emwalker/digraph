// @flow
import React from 'react'
import { GoRepo } from 'react-icons/go'

import type { Location, Router } from 'components/types'
import Page from 'components/ui/Page'
import SearchBox from 'components/ui/SearchBox'
import Columns from 'components/ui/Columns'
import LeftColumn from 'components/ui/LeftColumn'
import RightColumn from 'components/ui/RightColumn'
import SidebarList from 'components/ui/SidebarList'

/* eslint jsx-a11y/anchor-is-valid: 0 */

type Props = {
  location: Location,
  router: Router,
}

const LoadingPage = ({ location, router }: Props) => {
  const state = location.state || {}
  const { orgLogin, repoName, itemTitle } = state

  return (
    <Page>
      <nav aria-label="Breadcrumb" className="mb-1">
        <ol>
          <li className="breadcrumb-item">
            <GoRepo className="mr-1" />
            {' '}
            <a href="#">{orgLogin}</a>
          </li>
          <li
            className="breadcrumb-item breadcrumb-item-selected text-gray"
            aria-current="page"
          >
            { repoName }
          </li>
        </ol>
      </nav>
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
              orgLogin={orgLogin}
              placeholder="There are no parent topics for this topic."
              repoName={repoName}
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
