// @flow
import FetcherBase from '../FetcherBase'
import { type Operation, type Variables } from '../environment'

/* eslint class-methods-use-this: 0 */

class ClientFetcher extends FetcherBase {
  constructor(payloads: Object) {
    super()
    this.payloads = payloads
  }

  payloads: Object

  get url(): string {
    return '/graphql'
  }

  async fetch(operation: Operation, variables: Variables) {
    if (operation.name in this.payloads) {
      const payload = this.payloads[operation.name]
      delete this.payloads[operation.name]
      return payload
    }

    return super.fetch(operation, variables)
  }
}

export default ClientFetcher
