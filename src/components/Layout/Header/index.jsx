// @flow
import React from 'react'
import { Link } from 'found'
import Octicon from 'react-component-octicons'
import { pathOr } from 'ramda'

import type { UserType } from 'components/types'
import ViewerDropdown from './ViewerDropdown'
import GithubLogin from './GithubLogin'

const rootPath = pathOr('/', ['defaultRepository', 'rootTopic', 'resourcePath'])

type Props = {
  defaultOrganization: {
    defaultRepository: {
      rootTopic: {
        resourcePath: string,
      },
    },
  },
  viewer: UserType,
}

const Header = ({ defaultOrganization, viewer }: Props) => (
  <header className="Header pagehead pb-3">
    <div className="container-lg clearfix">
      <nav className="d-lg-flex float-left">
        <h1 className="h3 mt-2 text-normal">
          <Link
            to={defaultOrganization ? rootPath(defaultOrganization) : '/'}
            className="text-gray-dark n-link no-underline"
          >
            <span className="mr-2 d-inline-block">
              <Octicon name="git-branch" style={{ verticalAlign: 'middle' }} />
            </span>
            Digraph
          </Link>
        </h1>
      </nav>
      <div className="d-lg-flex float-right mt-1">
        <ul className="user-nav d-lg-flex list-style-none">
          <li>
            <summary className="p-2 d-inline"><a href="/about">About</a></summary>
          </li>
          <li className="dropdown">
            {viewer.isGuest
              ? <GithubLogin />
              : <ViewerDropdown viewer={viewer} />
            }
          </li>
        </ul>
      </div>
    </div>
  </header>
)

export default Header
