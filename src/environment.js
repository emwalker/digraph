// @flow
import { Environment, Network, RecordSource, Store } from 'relay-runtime'

type Fetcher = {
  fetch(): Promise<*>,
}

// eslint-disable-next-line import/prefer-default-export
export const createEnvironment = (fetcher: Fetcher) => (
  new Environment({
    network: Network.create((...args) => fetcher.fetch(...args)),
    store: new Store(new RecordSource()),
  })
)
