import 'core-js/'
import 'regenerator-runtime/runtime'
import BrowserProtocol from 'farce/lib/BrowserProtocol'
import createFarceRouter from 'found/lib/createFarceRouter'
import createRender from 'found/lib/createRender'
import { Resolver } from 'found-relay'
import React from 'react'
import ReactDOM from 'react-dom'
import { Environment, RecordSource, Store, Network } from 'relay-runtime'

import './css'
import {
  historyMiddlewares,
  routeConfig,
} from './router'
import fetchQuery from './fetchQuery'

const environment = new Environment({
  network: Network.create(fetchQuery),
  store: new Store(new RecordSource()),
})

const Router = createFarceRouter({
  historyProtocol: new BrowserProtocol(),
  historyMiddlewares,
  routeConfig,
  render: createRender({}),
})

ReactDOM.render(
  <Router resolver={new Resolver(environment)} />,
  document.getElementById('root'),
)
