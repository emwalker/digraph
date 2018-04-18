import { renderToString } from 'react-dom/server'
import Router from 'universal-router'

import routes from './routes'
import Api from './Api'

/* eslint no-param-reassign: 0 */
/* eslint consistent-return: 0 */

const router = new Router(routes)

export default function (options, callback) {
  callback = global[callback]

  const context = {
    api: Api.create({
      baseUrl: 'http://localhost:8080',
    }),
  }

  const payload = {
    uuid: options.uuid,
    app: null,
    title: null,
    meta: null,
    initial: null,
    error: null,
    redirect: null,
  }

  try {
    router.resolve({ path: options.url, ...context }).then((result) => {
      payload.app = renderToString(result.component)
      payload.title = result.title
      payload.initial = JSON.stringify({})
      return callback(payload)
    })
  } catch (e) {
    payload.error = e
    return callback(payload)
  }
}
