import { Location } from 'found'
import { OptionProps } from 'react-select'

export type Edge<T> = {
  node: T | null
} | null

export type Color = string

export type Connection<T> = {
  edges: readonly Edge<T>[] | null
}

export type EdgesTypeOf<C extends Connection<any>> = C['edges']
export type EdgeTypeOf<C extends Connection<any>> = NonNullable<EdgesTypeOf<C>>[number]
export type NodeTypeOf<C extends Connection<any>> = NonNullable<EdgeTypeOf<C>>['node']

export function liftEdges<T>(connection: Connection<T>) {
  return connection.edges || []
}

export function liftNodes<T>(connection: Connection<T> | undefined) {
  if (!connection) return []
  return liftEdges(connection).map((edge) => edge?.node || null) || []
}

type LocationState = {
  itemTitle: string,
}

export type LocationType = Pick<Location<LocationState>, 'pathname' | 'query' | 'search' | 'state'>

export interface TopicOption extends OptionProps {
  value: string,
  label: string,
  color: string,
}

export interface LinkOption extends OptionProps {
  value: string,
  label: string,
}

export type SynonymType = {
  locale: string,
  name: string,
}

export type AlertType = 'ERROR' | 'WARN' | 'SUCCESS' | '%future added value'
