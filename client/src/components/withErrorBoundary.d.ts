import React, { ComponentType } from 'react';
import { RouteRenderArgs } from 'found';
declare function withErrorBoundary<V>(Wrapped: ComponentType<any>): (routeProps: RouteRenderArgs) => React.JSX.Element;
export default withErrorBoundary;
