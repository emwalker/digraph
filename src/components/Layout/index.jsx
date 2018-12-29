// @flow
import React from 'react'
import type { Node } from 'react'

import Header from './Header'
import FlashMessages from '../FlashMessages'

type Props = {
  children?: Node,
  defaultOrganization: Object,
  viewer: Object,
}

/* eslint jsx-a11y/anchor-is-valid: 0 */

const Layout = ({ children, defaultOrganization, viewer }: Props) => (
  <div>
    <div className="container">
      <Header
        viewer={viewer}
        defaultOrganization={defaultOrganization}
      />
      <FlashMessages />
      { children }
    </div>
    <div className="container">
      <footer className="my-6 pt-4 border-top">
        <p className="mb-2">
          Available for use under the MIT{' '}
          <a href="https://github.com/emwalker/digraph/blob/master/LICENSE.md">license</a>.
          © Eric Walker.
        </p>
      </footer>
    </div>
  </div>
)

Layout.defaultProps = {
  children: null,
}

export default Layout
