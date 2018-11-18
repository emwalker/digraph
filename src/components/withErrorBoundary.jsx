import React from 'react'

import ErrorBoundary from './ui/ErrorBoundary'

export default Wrapped => props => (
  <ErrorBoundary>
    <Wrapped {...props} />
  </ErrorBoundary>
)
