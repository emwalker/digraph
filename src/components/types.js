// @flow

/* eslint no-use-before-define: 0 */

// Deprecated

export type AlertType = {
  id: string,
}

export type LinkConnection = {
  edges: LinkEdge[],
}

export type LinkEdge = {
  node: LinkType,
}

export type LinkType = {
  availableTopics: TopicConnection,
  id: string,
  loading: ?boolean,
  newlyAdded: boolean,
  parentTopics: TopicConnection,
  repository: RepositoryType,
  selectedTopics: TopicConnection,
  title: string,
  url: string,
}

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

export type OrganizationType = {
  id: string,
  login: string,
}

export type Relay = {
  environment: Object,
  refetch: Function,
}

export type RelayProps = {
  organization: OrganizationType,
  relay: Relay,
  topic: TopicType,
}

export type RepositoryConnection = {
  edges: RepositoryEdge[],
}

export type RepositoryEdge = {
  node: RepositoryType,
}

export type RepositoryType = {
  displayColor: string,
  displayName: string,
  fullName: string,
  id: string,
  isPrivate: boolean,
  name: string,
  organization: OrganizationType,
}

export type SearchResultItemConnection = {}

export type TopicConnection = {
  edges: TopicEdge[],
}

export type TopicEdge = {
  node: TopicType,
}

export type TopicType = {
  availableTopics: TopicConnection,
  childTopics: TopicConnection,
  description: ?string,
  displayName: string,
  id: string,
  links: LinkConnection,
  loading: ?boolean,
  name: string,
  newlyAdded: boolean,
  parentTopics: TopicConnection,
  repository: RepositoryType,
  resourcePath: string,
  search: SearchResultItemConnection,
  selectedTopics: TopicConnection,
  synonyms: any,
}

export type UserType = {
  avatarUrl: string,
  isGuest: boolean,
  name: string,
  repositories: RepositoryConnection,
  selectedRepository: ?RepositoryType,
}

export type ViewType = {
  currentRepository: RepositoryType,
  link: LinkType,
  topic: TopicType,
  topics: TopicConnection,
}

// Good
// @flow
type Edges<ConnectionType> = $NonMaybeType<$PropertyType<$NonMaybeType<ConnectionType>, 'edges'>>
type Edge<EdgesType> = $NonMaybeType<$ElementType<EdgesType, number>>
type Node<EdgeType> = $NonMaybeType<$PropertyType<EdgeType, 'node'>>

export type CollectionNode<ConnectionType> = Node<Edge<Edges<ConnectionType>>>
