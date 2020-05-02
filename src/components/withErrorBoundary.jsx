// @flow
import React, { type AbstractComponent } from 'react'

import ErrorBoundary from './ui/ErrorBoundary'

function withErrorBoundary<Props>(Wrapped: AbstractComponent<Props>) {
  return (props: Props) => (
    <ErrorBoundary>
      <Wrapped {...props} />
    </ErrorBoundary>
  )
}

export default withErrorBoundary
