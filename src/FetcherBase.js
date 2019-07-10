// @import
import 'isomorphic-fetch'

/* eslint class-methods-use-this: 0 */

class FetcherBase {
  get headers(): Object {
    return {
      'Content-Type': 'application/json',
    }
  }

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

export default FetcherBase
