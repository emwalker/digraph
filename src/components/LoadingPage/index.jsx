// @flow
import React from 'react'
import { GoRepo } from 'react-icons/go'

import SearchBox from 'components/ui/SearchBox'
import Columns from 'components/ui/Columns'
import LeftColumn from 'components/ui/LeftColumn'
import RightColumn from 'components/ui/RightColumn'
import SidebarList from 'components/ui/SidebarList'

/* eslint jsx-a11y/anchor-is-valid: 0 */

type Props = {
  location: {
    pathname: string,
    state: ?Object,
  }
}

const LoadingPage = ({ location }: Props) => {
  const state = location.state || {}
  const { orgLogin, repoName, itemTitle } = state

  return (
    <div className="px-3 px-md-6 px-lg-0">
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
          value=""
        />
      </div>
      <div>
        <Columns>
          <RightColumn>
            <SidebarList
              title="Parent topics"
              orgLogin={orgLogin}
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
    </div>
  )
}

export default LoadingPage
