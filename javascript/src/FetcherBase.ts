import 'isomorphic-fetch'
import { RequestParameters, Variables } from 'relay-runtime'

/* eslint class-methods-use-this: 0, no-underscore-dangle: 0 */

export class FetcherBase {
  private _url!: string

  public get url(): string {
    return this._url
  }

  public set url(value: string) {
    this._url = value
  }

  get headers() {
    return {
      'Content-Type': 'application/json',
    }
  }

  async fetch(request: RequestParameters, variables: Variables) {
    const { headers } = this

    const response = await fetch(this.url, {
      method: 'POST',
      headers,
      body: JSON.stringify({ query: request.text, variables }),
      credentials: 'include',
    })

    return response.json()
  }
}

export default FetcherBase
