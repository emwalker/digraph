// @flow
import 'isomorphic-fetch'

import type { Operation, Variables } from './environment'

/* eslint class-methods-use-this: 0 */

class FetcherBase {
  +url: string

  get headers(): Object {
    return {
      'Content-Type': 'application/json',
    }
  }

  async fetch(operation: Operation, variables: Variables) {
    const { headers } = this

    const response = await fetch(this.url, {
      method: 'POST',
      headers,
      body: JSON.stringify({ query: operation.text, variables }),
      credentials: 'include',
    })

    return response.json()
  }
}

export default FetcherBase
