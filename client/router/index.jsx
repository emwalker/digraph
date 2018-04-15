import React from 'react'
import { render } from 'react-dom'
import { Router, browserHistory } from 'react-router'
import { Promise } from 'when'

import toString from './toString'
import createRoutes from './routes'
import '../css'

/* eslint global-require: 0 */

export function run() {
  // init promise polyfill
  window.Promise = window.Promise || Promise
  // init fetch polyfill
  window.self = window
  require('whatwg-fetch')

  render(
    <Router history={browserHistory}>{createRoutes({ first: { time: true } })}</Router>,
    document.getElementById('app'),
  )
}

// Export it to render on the Golang server, keep the name sync with -
// https://github.com/olebedev/go-starter-kit/blob/master/server/react.go#L65
export const renderToString = toString

// Style live reloading
if (module.hot) {
  let c = 0
  module.hot.accept('../css', () => {
    require('../css')
    const a = document.createElement('a')
    const link = document.querySelector('link[rel="stylesheet"]')
    a.href = link.href
    a.search = `?${c}`
    c += 1
    link.href = a.href
  })
}
