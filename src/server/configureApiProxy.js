// @flow
import { createProxyMiddleware } from 'http-proxy-middleware'

/* eslint no-console: 0, implicit-arrow-linebreak: 0 */
const graphqlApiBaseUrl = process.env.DIGRAPH_API_BASE_URL || 'http://localhost:8080'

export const basicAuthSecret = (viewerId: string, sessionId: string) =>
  Buffer.from(`${viewerId}:${sessionId}`).toString('base64')

export default (app: Object) => {
  app.post(
    '/graphql',
    createProxyMiddleware({
      target: graphqlApiBaseUrl,
      changeOrigin: true,
      secure: false,
      onProxyReq(proxyReq, req) {
        const { user } = req

        if (user) {
          const { id, sessionId } = user
          const secret = basicAuthSecret(id, sessionId)
          proxyReq.setHeader('Authorization', `Basic ${secret}`)
        } else {
          console.log('No user found with the request, omitting basic auth header')
        }
      },
      onError(err) {
        console.log('There was a problem proxying request to api server:', err)
      },
    }),
  )

  app.get(
    '/_ah/health',
    createProxyMiddleware({
      target: graphqlApiBaseUrl,
      changeOrigin: true,
      secure: false,
    }),
  )

  return app
}
