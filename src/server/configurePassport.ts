import passport from 'passport'
import { Express } from 'express'
import { createClient } from 'redis'
import session from 'express-session'
import connectRedis from 'connect-redis'
import cookieParser from 'cookie-parser'
import bodyParser from 'body-parser'

import deleteSessionMutation, { Input } from 'mutations/deleteSessionMutation'
import { createEnvironment } from '../environment'
import withGithub from './auth/withGithub'
import { FetcherBase } from '../FetcherBase'

/* eslint no-console: 0 */

const RedisStore = connectRedis(session)

const options: connectRedis.RedisStoreOptions = {
  client: undefined,
}

if (process.env.DIGRAPH_REDIS_PASSWORD) {
  options.client = createClient({
    host: process.env.DIGRAPH_NODE_REDIS_HOST,
    password: process.env.DIGRAPH_REDIS_PASSWORD,
  })
} else {
  options.client = createClient()
}

export default (app: Express, fetcher: FetcherBase): Express => {
  const environment = createEnvironment(fetcher)

  app.use(session({
    store: new RedisStore(options),
    secret: process.env.DIGRAPH_COOKIE_SECRET || 'keyboard cat',
    resave: true,
    saveUninitialized: true,
    // Expire in one month
    cookie: { maxAge: 1000 * 3600 * 24 * 30 },
  }))

  app
    .use(passport.initialize())
    .use(passport.session())
    .use(bodyParser.urlencoded({ extended: true }))
    .use(cookieParser())

  withGithub(app, environment)

  app.get('/logout', (req, res) => {
    const sessionId = req.user?.sessionId
    if (!sessionId) {
      console.log('No session id, cannot log out:', sessionId)
      return
    }

    const input: Input = { sessionId }
    deleteSessionMutation(
      environment,
      input,
      {
        onCompleted() {
          console.log('Deleted session for user', req.user?.id)
          req.logout()
          res.redirect('/')
        },

        onError(error: Error) {
          const userId = req.user?.id
          console.log(`Failed to delete session for user ${userId}`, error)
          req.logout()
        },
      },
    )
  })

  passport.serializeUser((viewer, done) => {
    console.log('serializeUser', viewer)
    done(null, [viewer.id, viewer.sessionId])
  })

  passport.deserializeUser((ids: string[], done) => {
    const [id, sessionId] = ids
    const viewer = { id, sessionId }
    console.log('deserializeUser', id)
    done(null, viewer)
  })

  return app
}
