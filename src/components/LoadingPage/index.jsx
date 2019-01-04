// @flow
import React from 'react'
import Octicon from 'react-component-octicons'

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
            { orgLogin }
          </li>
          <li
            className="breadcrumb-item breadcrumb-item-selected text-gray"
            aria-current="page"
          >
            { repoName }
          </li>
        </ol>
      </nav>
      <div className="Subhead">
        <div className="Subhead-heading">
          { itemTitle }
        </div>
      </div>
    </div>
  )
}

export default LoadingPage
