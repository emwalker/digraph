import React, { ComponentType } from 'react'
import { RouteRenderArgs, RenderProps } from 'found'

import ErrorBoundary from './ui/ErrorBoundary'

interface ViewRenderProps extends RenderProps {
  view: any,
}

function withErrorBoundary(Wrapped: ComponentType<any>) {
  return ({ props }: RouteRenderArgs) => {
    const renderProps = props as ViewRenderProps
    const view = renderProps?.view
    return (
      view && (
      <ErrorBoundary>
        <Wrapped view={view} />
      </ErrorBoundary>
      )
    )
  }
}

export default withErrorBoundary
