import { GraphQLTaggedNode } from 'relay-runtime'
import { RouteObjectBase, RouteMatch, RenderProps } from 'found'

declare module 'found' {
  export interface RouteObjectBase {
    getQuery?: (match: RouteMatch) => GraphQLTaggedNode,
  }
}
