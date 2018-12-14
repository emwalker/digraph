// @flow
import React from 'react'
import { Link } from 'found'
import Octicon from 'react-component-octicons'
import { pathOr } from 'ramda'

import ViewerDropdown from './ViewerDropdown'
import GithubLogin from './GithubLogin'

const rootPath = pathOr('/', ['defaultRepository', 'rootTopic', 'resourcePath'])

type Props = {
  viewer: ?{
    defaultRepository: {
      rootTopic: {
        resourcePath: string,
      },
    },
  },
}

const Header = ({ viewer }: Props) => (
  <header className="Header pagehead pb-3">
    <div className="container-lg clearfix">
      <nav className="d-lg-flex float-left">
        <h1 className="h3 mt-2 text-normal">
          <Link
            to={viewer ? rootPath(viewer) : '/'}
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
          <li className="dropdown">
            {viewer
              ? <ViewerDropdown viewer={viewer} />
              : <GithubLogin />
            }
          </li>
        </ul>
      </div>
    </div>
  </header>
)

export default Header
