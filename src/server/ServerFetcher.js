import FetcherBase from '../FetcherBase'

import { authHeaders } from './configureApiProxy'

/* eslint class-methods-use-this: 0, no-console: 0 */

const graphqlApiBaseUrl = process.env.DIGRAPH_API_BASE_URL || 'http://localhost:8080'

class ServerFetcher extends FetcherBase {
  constructor() {
    super()
    this.payloads = {}
  }

  get url(): string {
    return `${graphqlApiBaseUrl}/graphql`
  }

  get headers(): Object {
    return {
      'Content-Type': 'application/json',
      ...authHeaders,
    }
  }

  clear() {
    this.payloads = {}
  }

  async fetch(operation, variables, config) {
    console.log('Quering from the node server:', operation.name)
    const payload = await super.fetch(operation, variables, config)
    this.payloads[operation.name] = payload
    return payload
  }

  toJSON() {
    return this.payloads
  }
}

export default ServerFetcher
