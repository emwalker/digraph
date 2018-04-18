require('isomorphic-fetch')

// eslint-disable-next-line import/first
import {
  Environment,
  Network,
  RecordSource,
  Store,
  fetchQuery,
  commitMutation,
  commitLocalUpdate,
} from 'relay-runtime'

/**
 * Creates a set of helper methods for working with REST and/or GraphQL APIs.
 */
function create({ baseUrl, headers = {} }) {
  // Default options for the Fetch API
  // https://developer.mozilla.org/docs/Web/API/Fetch_API/Using_Fetch
  const isLocal = /localhost/.test(baseUrl)

  const defaults = {
    mode: 'cors',
    credentials: isLocal ? 'same-origin' : 'include',
    headers: {
      ...headers,
      Accept: 'application/json',
      'Content-Type': 'application/json',
    },
  }

  // Configure Relay environment
  const environment = new Environment({
    handlerProvider: null,
    network: Network.create(
      /* , variables, cacheConfig, uploadables */
      operation => (
        fetch(`${baseUrl}/graphql`, {
          ...defaults,
          method: 'POST',
          // body: JSON.stringify({
          //   query: operation.text,
          //   variables,
          // }),
          body: operation.text,
        }).then(response => response.json())
      ),
    ),
    store: new Store(new RecordSource()),
  })

  return {
    environment,
    fetch: (url, options) => fetch(`${baseUrl}${url}`, {
      ...defaults,
      ...options,
      headers: {
        ...defaults.headers,
        ...(options && options.headers),
      },
    }),
    fetchQuery: fetchQuery.bind(undefined, environment),
    commitMutation: commitMutation.bind(undefined, environment),
    commitLocalUpdate: commitLocalUpdate.bind(undefined, environment),
  }
}

export default { create }
