import React from 'react'
import { renderToString } from 'react-dom/server'
import { match, RouterContext } from 'react-router'
import Helmet from 'react-helmet'
import createRoutes from './routes'

/* eslint consistent-return: 0 */
/* eslint no-param-reassign: 0 */

const rules = (options) => ({
  routes: createRoutes({ first: { time: false } }),
  location: options.url,
})

/**
 * Handle HTTP request at Golang server
 *
 * @param   {Object}   options  request options
 * @param   {Function} cbk      response callback
 */
export default function (options, cbk) {
  cbk = global[cbk]
  const result = {
    uuid: options.uuid,
    app: null,
    title: null,
    meta: null,
    initial: null,
    error: null,
    redirect: null,
  }

  try {
    match(rules(options), (error, redirectLocation, renderProps) => {
      try {
        if (error) {
          result.error = error
        } else if (redirectLocation) {
          result.redirect = redirectLocation.pathname + redirectLocation.search
        } else {
          result.app = renderToString(
            <RouterContext {...renderProps} />,
          )
          const { title, meta } = Helmet.rewind()
          result.title = title.toString()
          result.meta = meta.toString()
          result.initial = JSON.stringify({})
        }
      } catch (e) {
        result.error = e
      }
      return cbk(result)
    })
  } catch (e) {
    result.error = e
    return cbk(result)
  }
}
