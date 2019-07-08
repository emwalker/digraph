// @flow
import requestProxy from 'http-proxy-middleware'

/* eslint no-console: 0 */
const graphqlApiBaseUrl = process.env.DIGRAPH_API_BASE_URL || 'http://localhost:8080'
const basicAuthUsername = process.env.DIGRAPH_BASIC_AUTH_USERNAME || ''
const basicAuthPassword = process.env.DIGRAPH_BASIC_AUTH_PASSWORD || ''
const authSecret = Buffer.from(`${basicAuthUsername}:${basicAuthPassword}`).toString('base64')

export const authHeaders = {
  Authorization: `Basic ${authSecret}`,
}

export default (app: Object) => {
  if (!basicAuthUsername || !basicAuthPassword) console.log('Basic auth username and password not set, proxying will fail')

  return app.post(
    '/graphql',
    requestProxy({
      target: graphqlApiBaseUrl,
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
