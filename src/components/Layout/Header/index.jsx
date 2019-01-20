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
  <header
    className="Header clearfix mb-3 d-flex px-4 py-2 box-shadow"
  >
    <nav className="flex-self-center">
      <h1 className="h3 text-normal">
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
    <nav className="user-nav flex-self-center">
      <a className="text-gray-dark px-2" href="/about">About</a>
      {viewer.isGuest
        ? <GithubLogin />
        : <ViewerDropdown viewer={viewer} />
      }
    </nav>
  </header>
)

export default Header
