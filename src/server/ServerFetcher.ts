import { RequestParameters, Variables } from 'relay-runtime'
import { FetcherBase } from '../FetcherBase'

import { basicAuthSecret } from './configureApiProxy'

type Headers = {
  Authorization?: string,
  'Content-Type': string,
}

/* eslint class-methods-use-this: 0, no-console: 0 */

const graphqlApiBaseUrl = process.env.DIGRAPH_API_BASE_URL || 'http://localhost:8080'

class ServerFetcher extends FetcherBase {
  constructor() {
    super()
    this.payloads = {}
  }

  payloads: { [key: string]: string }

  sessionId: string | undefined | null

  viewerId: string | undefined | null

  setBasicAuth(viewerId: string, sessionId: string) {
    this.viewerId = viewerId
    this.sessionId = sessionId
  }

  get url(): string {
    return `${graphqlApiBaseUrl}/graphql`
  }

  get headers() {
    const headers: Headers = {
      'Content-Type': 'application/json',
    }

    if (this.viewerId && this.sessionId) {
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

  async fetch(request: RequestParameters, variables: Variables) {
    console.log('Quering from the node server:', request.name)
    const payload = await super.fetch(request, variables)
    this.payloads[request.name] = payload
    return payload
  }

  toJSON() {
    return this.payloads
  }
}

export default ServerFetcher
