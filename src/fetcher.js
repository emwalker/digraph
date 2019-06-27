import 'isomorphic-fetch'

import { authHeaders } from './server/configureApiProxy'

/* eslint class-methods-use-this: 0, no-console: 0 */

class FetcherBase {
  async fetch(operation, variables) {
    const { headers } = this

    const response = await fetch(this.url, {
      method: 'POST',
      headers,
      body: JSON.stringify({ query: operation.text, variables }),
    })

    return response.json()
  }
}

export class ServerFetcher extends FetcherBase {
  constructor() {
    super()
    this.payloads = {}
  }

  get url(): string {
    return 'http://localhost:8080/graphql'
  }

  get headers(): Object {
    return {
      'Content-Type': 'application/json',
      ...authHeaders,
    }
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

export class ClientFetcher extends FetcherBase {
  constructor(payloads) {
    super()
    this.payloads = payloads
  }

  get headers(): Object {
    return {
      'Content-Type': 'application/json',
    }
  }

  get url(): string {
    return 'http://localhost:3001/graphql'
  }

  async fetch(operation, variables, config) {
    if (operation.name in this.payloads) {
      const payload = this.payloads[operation.name]
      delete this.payloads[operation.name]
      return payload
    }

    return super.fetch(operation, variables, config)
  }
}
