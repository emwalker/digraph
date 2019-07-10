import FetcherBase from '../FetcherBase'

import { basicAuthSecret } from './configureApiProxy'

/* eslint class-methods-use-this: 0, no-console: 0 */

const graphqlApiBaseUrl = process.env.DIGRAPH_API_BASE_URL || 'http://localhost:8080'

class ServerFetcher extends FetcherBase {
  constructor() {
    super()
    this.payloads = {}
  }

  setBasicAuth(viewerId, sessionId) {
    this.viewerId = viewerId
    this.sessionId = sessionId
  }

  get url(): string {
    return `${graphqlApiBaseUrl}/graphql`
  }

  get headers(): Object {
    const headers = {
      'Content-Type': 'application/json',
    }

    if (this.viewerId) {
      const secret = basicAuthSecret(this.viewerId, this.sessionId)
      headers.Authorization = `Basic ${secret}`
    }

    return headers
  }

  clear() {
    this.payloads = {}
    this.viewerId = null
    this.sessionId = null
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
