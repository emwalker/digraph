import { RequestParameters, Variables } from 'relay-runtime'

import { FetcherBase } from '../FetcherBase'

/* eslint class-methods-use-this: 0 */

type PayloadsType = { [key: string]: string }

class ClientFetcher extends FetcherBase {
  constructor(payloads: PayloadsType) {
    super()
    this.payloads = payloads
  }

  payloads: PayloadsType

  get url(): string {
    return '/graphql'
  }

  async fetch(request: RequestParameters, variables: Variables) {
    if (request.name in this.payloads) {
      const payload = this.payloads[request.name]
      delete this.payloads[request.name]
      return payload
    }

    return super.fetch(request, variables)
  }
}

export default ClientFetcher
