import { Environment, Network, RecordSource, Store } from 'relay-runtime'

import { FetcherBase } from './FetcherBase'

// eslint-disable-next-line import/prefer-default-export
export const createEnvironment = (fetcher: FetcherBase): Environment => (
  new Environment({
    network: Network.create((request, variables) => fetcher.fetch(request, variables)),
    store: new Store(new RecordSource()),
  })
)
