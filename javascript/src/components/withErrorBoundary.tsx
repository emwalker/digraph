import React, { ComponentType } from 'react'
import { RouteRenderArgs, RenderProps } from 'found'

import ErrorBoundary from './ui/ErrorBoundary'

interface ViewRenderProps<V> extends RenderProps {
  view: V,
}

function withErrorBoundary<V>(Wrapped: ComponentType<any>) {
  return (routeProps: RouteRenderArgs) => {
    const { props, match } = routeProps
    const renderProps = props as ViewRenderProps<V>
    const view = renderProps?.view
    return (
      view && (
      <ErrorBoundary>
        <Wrapped view={view} match={match} />
      </ErrorBoundary>
      )
    )
  }
}

export default withErrorBoundary
