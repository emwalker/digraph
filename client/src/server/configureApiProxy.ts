import { Express, Request } from 'express'
import { createProxyMiddleware } from 'http-proxy-middleware'

export interface IGetUserAuthInfoRequest extends Request {
  user?: {
    id?: string,
    sessionId?: string,
  },
}

/* eslint no-console: 0, implicit-arrow-linebreak: 0 */
const graphqlApiBaseUrl = process.env.DIGRAPH_API_BASE_URL || 'http://localhost:8080'

export const basicAuthSecret = (viewerId: string, sessionId: string) =>
  Buffer.from(`${viewerId}:${sessionId}`).toString('base64')

export default (app: Express) => {
  app.post(
    '/graphql',
    createProxyMiddleware({
      target: `${graphqlApiBaseUrl}/graphql`,
      changeOrigin: true,
      pathFilter: '/graphql',
      secure: false,
      on: {
        proxyReq(proxyReq, req: IGetUserAuthInfoRequest) {
          const { user } = req
          console.log('proxyReq config', user)

          if (user && user.id && user.sessionId) {
            const { id, sessionId } = user
            const secret = basicAuthSecret(id, sessionId)
            proxyReq.setHeader('Authorization', `Basic ${secret}`)
          } else {
            console.log('no user found with the request, omitting basic auth header')
          }
        },
        error(err) {
          console.log('problem proxying request to api server:', err)
        },
      }
    }),
  )

  app.get(
    '/_ah/health',
    createProxyMiddleware({
      target: `${graphqlApiBaseUrl}/_ah/health`,
      changeOrigin: true,
      secure: false,
    }),
  )

  return app
}
