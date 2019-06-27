// @flow
import requestProxy from 'http-proxy-middleware'

/* eslint no-console: 0 */

const userId = process.env.DIGRAPH_BASIC_AUTH_USERNAME || 'no-such-user'
const password = process.env.DIGRAPH_BASIC_AUTH_PASSWORD || 'no-such-password'
const authSecret = Buffer.from(`${userId}:${password}`).toString('base64')

export const authHeaders = {
  Authorization: `Basic ${authSecret}`,
}

export default (app: Object) => {
  if (!userId || !password) console.log('Basic auth username and password not set, proxying will fail')

  return app.post(
    '/graphql',
    requestProxy({
      target: 'http://localhost:8080',
      changeOrigin: true,
      secure: false,
      onProxyReq(proxyReq) {
        proxyReq.setHeader('Authorization', authHeaders.Authorization)
      },
      onError(err) {
        console.log('There was a problem proxying request to api server:', err)
      },
    }),
  )
}
