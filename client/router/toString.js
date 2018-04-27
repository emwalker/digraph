import { getFarceResult } from 'found/lib/server'
import { renderToString } from 'react-dom/server'
import serialize from 'serialize-javascript'

import {
  createResolver,
  historyMiddlewares,
  render,
  routeConfig,
} from './router'
import { ServerFetcher } from './fetcher'

/* eslint no-param-reassign: 0 */

const PORT = 5000

export default async (options, callback) => {
  callback = global[callback]

  const fetcher = new ServerFetcher(`http://localhost:${PORT}/graphql`)

  const payload = {
    uuid: options.uuid,
    app: null,
    title: null,
    meta: null,
    initial: null,
    error: null,
    redirect: null,
  }

  const { redirect, element } = await getFarceResult({
    url: options.url,
    historyMiddlewares,
    routeConfig,
    resolver: createResolver(fetcher),
    render,
  })

  try {
    if (redirect) {
      payload.redirect = redirect.url
    } else {
      payload.title = 'Digraffe'
      payload.app = renderToString(element)
      payload.initial = serialize(fetcher, { isJSON: true })
    }
  } catch (e) {
    payload.error = e
  }
  return callback(payload)
}
