// @flow
import passport from 'passport'
import { createClient } from 'redis'
import session from 'express-session'
import connectRedis from 'connect-redis'
import cookieParser from 'cookie-parser'
import bodyParser from 'body-parser'

import deleteSessionMutation, { type Input } from 'mutations/deleteSessionMutation'
import { createEnvironment } from '../environment'
import withGithub from './auth/withGithub'
import type { Fetcher } from '../environment'
import type { App } from './types'

/* eslint no-console: 0 */

const RedisStore = connectRedis(session)

let client

if (process.env.DIGRAPH_REDIS_PASSWORD) {
  client = createClient({
    host: process.env.DIGRAPH_NODE_REDIS_HOST,
    password: process.env.DIGRAPH_REDIS_PASSWORD,
  })
} else {
  client = createClient()
}

export default (app: App, fetcher: Fetcher) => {
  const environment = createEnvironment(fetcher)

  app.use(session({
    store: new RedisStore({ client }),
    secret: process.env.DIGRAPH_COOKIE_SECRET || 'keyboard cat',
    resave: true,
    saveUninitialized: true,
  }))

  app
    .use(passport.initialize())
    .use(passport.session())
    .use(bodyParser.urlencoded({ extended: true }))
    .use(cookieParser())

  withGithub(app, environment)

  app.get('/logout', (req, res) => {
    const input: Input = { sessionId: req.user.sessionId }
    deleteSessionMutation(
      environment,
      input,
      {
        onCompleted() {
          console.log('Deleted session for user', req.user.id)
          req.logout()
          res.redirect('/')
        },

        onError(error) {
          const errors = error.source && error.source.errors
          const userId = req.user && req.user.id
          console.log(`Failed to delete session for user ${userId}`, error, errors)
          req.logout()
        },
      },
    )
  })

  passport.serializeUser((viewer, done) => {
    console.log('serializeUser', viewer)
    done(null, [viewer.id, viewer.sessionId])
  })

  passport.deserializeUser((ids, done) => {
    const [id, sessionId] = ids
    const viewer = { id, sessionId }
    console.log('deserializeUser', id)
    done(null, viewer)
  })

  return app
}
