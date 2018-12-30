// @flow

export type OrganizationType = {
  id: string,
}

export type TopicType = {
  id: string,
  name: string,
  newlyAdded: boolean,
  resourcePath: string,
}

export type LinkType = {
  id: string,
  newlyAdded: boolean,
  title: string,
  url: string,
}

export type RelayProps = {
  organization: OrganizationType,
  relay: {
    environment: Object,
  },
  topic: TopicType,
}
