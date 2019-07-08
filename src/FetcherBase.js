// @import
import 'isomorphic-fetch'

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

export default FetcherBase
