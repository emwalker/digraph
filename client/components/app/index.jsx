import React from 'react'
import Helmet from 'react-helmet'

/* eslint react/prop-types: 0 */

export default ({ children }) => (
  <div>
    <Helmet title="Go + React + Redux = rocks!" />
    {children}
  </div>
)
