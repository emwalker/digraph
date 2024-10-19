import 'core-js'
import 'regenerator-runtime/runtime'
import React from 'react'
import express from 'express'
import { getFarceResult } from 'found/server'
import compression from 'compression'
import { createStore } from 'redux'
import { Provider } from 'react-redux'
import path from 'path'

import renderFullPage from './renderFullPage'
import ServerFetcher from './ServerFetcher'
import configurePassport from './configurePassport'
import configureApiProxy from './configureApiProxy'

import {
  createResolver,
  createRouteConfig,
  historyMiddlewares,
  render,
} from '../router'

/* eslint no-console: 0, react/jsx-filename-extension: 0 */

const publicDir = path.join(__dirname, 'public/static')
const imagesDir = path.join(__dirname, 'public/images')

// @ts-expect-error
if (typeof window === 'undefined') global.window = {}

// eslint-disable-next-line import/no-dynamic-require
const oneMonth = 2629800000
const assets = require(process.env.RAZZLE_ASSETS_MANIFEST || '')
const reducer = (state: any) => state
const fetcher = new ServerFetcher()

// eslint-disable-next-line import/no-mutable-exports
let app = express()

app = configurePassport(app, fetcher)
app = configureApiProxy(app)

app
  .disable('x-powered-by')
  .use(compression())
  .use('/static/images', express.static(imagesDir, { maxAge: oneMonth }))
  .use('/static', express.static(publicDir, { maxAge: oneMonth }))
  .use('/robots.txt', (req, res) => {
    res.type('text/plain')
    res.send('User-agent: *\nAllow: /\n')
  })

app.get('*', async (req, res): Promise<void> => {
  fetcher.clear()

  try {
    const preloadedState = { viewer: req.user }
    console.log('preloadedState', preloadedState)

    if (req.user) {
      const { id, sessionId } = req.user
      if (id && sessionId) fetcher.setBasicAuth(id, sessionId)
    }

    const store = createStore(reducer, preloadedState)
    const routeConfig = createRouteConfig(store)
    const resolver = createResolver(fetcher)

    const result = await getFarceResult({
      historyMiddlewares,
      render,
      resolver,
      routeConfig,
      url: req.url,
    })

    if ('redirect' in result) {
      res.redirect(302, result.redirect.url)
      return
    }

    const { element, status } = result

    const wrapped = (
      <Provider store={store}>
        {element}
      </Provider>
    )

    const html = await renderFullPage(assets, fetcher, wrapped, preloadedState)
    res.status(status).send(html)
  } catch (e) {
    console.log('error', 'There was a problem', e)
    res.status(500).send('There was a problem on the server')
  }
})

export default app
