// @flow
import { Environment } from 'react-relay'

export type LocationDescriptor = {
  pathname: string,
  state: {
    orgLogin: string,
    repoName: ?string,
    itemTitle: string,
  },
}

export type Option = {
  value: string,
  label: string,
  color?: string,
}

export type Relay = {
  environment: Environment,
  refetch: Function,
}

export type Edges<ConnectionType> = $NonMaybeType<$PropertyType<$NonMaybeType<ConnectionType>, 'edges'>>
export type Edge<EdgesType> = $NonMaybeType<$ElementType<EdgesType, number>>
export type Node<EdgeType> = $NonMaybeType<$PropertyType<EdgeType, 'node'>>
export type CollectionNode<ConnectionType> = Node<Edge<Edges<ConnectionType>>>

export type Match = {
  location: {
    pathname: string,
  },
}
