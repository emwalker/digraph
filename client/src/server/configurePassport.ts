import passport from 'passport'
import { Express, urlencoded, RequestHandler } from 'express'
import { createClient } from 'redis'
import session from 'express-session'
import store from 'connect-redis'
import cookieParser from 'cookie-parser'

import deleteSessionQuery from 'mutations/deleteSessionMutation'
import { deleteSessionMutation } from '__generated__/deleteSessionMutation.graphql'
import { createEnvironment } from '../environment'
import withGithub from './auth/withGithub'
import { FetcherBase } from '../FetcherBase'
import { commitMutation } from 'react-relay'
import { Environment, MutationConfig } from 'relay-runtime'

// @ts-expect-error
const RedisStore = store(session)
const redisClient = createClient({
  url: process.env.DIGRAPH_NODE_REDIS_URL || 'redis://localhost:6379',
  legacyMode: true,
})
redisClient.connect().catch(console.error)

type Config = Omit<MutationConfig<deleteSessionMutation>, 'mutation'>

function deleteSession(environment: Environment, config: Config) {
  return commitMutation<deleteSessionMutation>(environment,
    { ...config, mutation: deleteSessionQuery })
}

export default (app: Express, fetcher: FetcherBase): Express => {
  const environment = createEnvironment(fetcher)

  app.use(session({
    // @ts-expect-error
    store: new RedisStore({
      client: redisClient,
      logErrors: true,
    }),
    secret: process.env.DIGRAPH_COOKIE_SECRET || 'keyboard cat',
    resave: true,
    saveUninitialized: true,
    secure: process.env.NODE_ENV == 'production',
    // Expire in one month
    cookie: { maxAge: 1000 * 3600 * 24 * 30 },
  }))

  app
    .use(passport.initialize())
    .use(passport.session())
    .use(urlencoded({ extended: true }) as RequestHandler)
    .use(cookieParser())

  withGithub(app, environment)

  app.get('/logout', (req, res) => {
    const sessionId = req.user?.sessionId
    if (!sessionId) {
      // eslint-disable-next-line no-console
      console.log('No session id, cannot log out:', sessionId)
      return
    }

    const variables = { input: { sessionId } }

    const onCompleted = () => {
      console.log('Deleted session for user', req.user?.id)
      req.logout()
      res.redirect('/')
    }

    const onError = (error: Error) => {
      const userId = req.user?.id
      // eslint-disable-next-line no-console
      console.log(`Failed to delete session for user ${userId}`, error)
      req.logout()
    }

    deleteSession(environment, { variables, onCompleted, onError })
  })

  passport.serializeUser((viewer, done) => {
    // eslint-disable-next-line no-console
    console.log('serializeUser', viewer)
    done(null, [viewer.id, viewer.sessionId])
  })

  passport.deserializeUser((ids: string[], done) => {
    const [id, sessionId] = ids
    const viewer = { id, sessionId }
    // eslint-disable-next-line no-console
    console.log('deserializeUser', id)
    done(null, viewer)
  })

  return app
}
