import FetcherBase from '../FetcherBase'

/* eslint class-methods-use-this: 0 */

class ClientFetcher extends FetcherBase {
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
    return '/graphql'
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

export default ClientFetcher
