// @flow

export type OrganizationType = {
  id: string,
}

export type TopicType = {
  name: string,
  id: string,
  resourcePath: string,
}

export type LinkType = {
  title: string,
  url: string,
  id: string,
}

export type RelayProps = {
  organization: OrganizationType,
  relay: {
    environment: Object,
  },
  topic: TopicType,
}
