// @flow
import React from 'react'
import type { Node } from 'react'

type Props = {
  children?: Node,
}

const Layout = ({ children }: Props) => (
  <div>
    <div className="container">
      <div className="pagehead">
        <h1>Digraph</h1>
      </div>
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
