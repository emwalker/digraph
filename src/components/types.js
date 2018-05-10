// @flow

export type OrganizationType = {
  resourceId: string,
}

export type TopicType = {
  name: string,
  resourceId: string,
  resourcePath: string,
}

export type LinkType = {
  title: string,
  url: string,
  resourceId: string,
}

export type RelayProps = {
  organization: {
    id: string,
    resourceId: string,
  },
  relay: {
    environment: Object,
  },
  topic: {
    id: string,
    resourceId: string,
  },
}
