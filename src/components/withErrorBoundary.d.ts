import { ComponentType } from 'react';
import { RouteRenderArgs } from 'found';
declare function withErrorBoundary(Wrapped: ComponentType<any>): ({ props }: RouteRenderArgs) => any;
export default withErrorBoundary;
