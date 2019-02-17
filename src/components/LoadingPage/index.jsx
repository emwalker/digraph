// @flow
import React from 'react'
import Octicon from 'react-component-octicons'

import SearchBox from 'components/ui/SearchBox'

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
    <div>
      <nav aria-label="Breadcrumb" className="mb-1">
        <ol>
          <li className="breadcrumb-item">
            <Octicon name="repo" className="mr-1" />
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
      <div className="page-placeholder" />
    </div>
  )
}

export default LoadingPage
