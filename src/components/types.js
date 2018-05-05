// @flow

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
