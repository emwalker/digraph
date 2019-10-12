// @flow
import { Environment, Network, RecordSource, Store } from 'relay-runtime'

export type Operation = {|
  name: string,
  text: string,
|}

export type Variables = {||}

export type Fetcher = {
  fetch(Operation, Variables): Promise<*>,
}

// eslint-disable-next-line import/prefer-default-export
export const createEnvironment = (fetcher: Fetcher): Environment => (
  new Environment({
    network: Network.create((...args) => fetcher.fetch(...args)),
    store: new Store(new RecordSource()),
  })
)
