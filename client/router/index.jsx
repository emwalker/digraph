import React from 'react'
import BrowserProtocol from 'farce/lib/BrowserProtocol'
import createFarceRouter from 'found/lib/createFarceRouter'
import ReactDOM from 'react-dom'
import { Promise } from 'when'

import { ClientFetcher } from './fetcher'
import toString from './toString'

import {
  createResolver,
  historyMiddlewares,
  render,
  routeConfig,
} from './router'

import '../css'

/* eslint no-underscore-dangle: 0 */

export function run() {
  window.Promise = window.Promise || Promise
  window.self = window

  const fetcher = new ClientFetcher('/graphql', window.__RELAY_PAYLOADS__)
  const resolver = createResolver(fetcher)

  const Router = createFarceRouter({
    historyProtocol: new BrowserProtocol(),
    historyMiddlewares,
    routeConfig,
    resolver,
    render,
  })

  ReactDOM.render(
    <Router resolver={resolver} />,
    document.getElementById('app'),
  )
}

// Export it to render on the Golang server, keep the name sync with
// server/react.go
export const renderToString = toString

// Live style reloading
if (module.hot) {
  let c = 0
  module.hot.accept('../css', () => {
    // eslint-disable-next-line global-require
    require('../css')
    const a = document.createElement('a')
    const link = document.querySelector('link[rel="stylesheet"]')
    a.href = link.href
    a.search = `?${c}`
    c += 1
    link.href = a.href
  })
}
