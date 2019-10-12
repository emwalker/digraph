// @flow
import React, { type AbstractComponent } from 'react'

import ErrorBoundary from './ui/ErrorBoundary'

function withErrorBoundary<Config, Props>(Wrapped: AbstractComponent<Config>) {
  return (props: Props) => (
    <ErrorBoundary>
      <Wrapped {...props} />
    </ErrorBoundary>
  )
}

export default withErrorBoundary
