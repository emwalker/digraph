import passport from 'passport'
import { Strategy } from 'passport-github'
import session from 'express-session'
import connectRedis from 'connect-redis'
import cookieParser from 'cookie-parser'
import bodyParser from 'body-parser'

import createSessionMutation from 'mutations/createSessionMutation'
import deleteSessionMutation from 'mutations/deleteSessionMutation'
import { createEnvironment } from '../environment'

/* eslint no-console: 0 */

const RedisStore = connectRedis(session)

const primaryOrFirstEmail = (emails) => {
  const matches = emails.filter(({ primary }) => primary)
  if (matches.length) return matches[0].value
  if (emails.length) return emails[0].value
  return null
}

export default (app, fetcher) => {
  const environment = createEnvironment(fetcher)

  const onGithubAuthSuccess = async (accessToken, refreshToken, profile, done) => {
    console.log('GitHub login succeeded, getting viewer id')

    // eslint-disable-next-line camelcase
    const { displayName, emails, username, _json: { avatar_url } } = profile
    const email = primaryOrFirstEmail(emails)

    const input = {
      githubAvatarUrl: avatar_url,
      githubUsername: username,
      name: displayName,
      primaryEmail: email,
      serverSecret: process.env.DIGRAPH_SERVER_SECRET || 'keyboard cat',
    }

    createSessionMutation(environment, [], input, {
      onCompleted(payload) {
        if (!payload.createSession) {
          console.log('createSession field missing from response:', payload)
          done(null, null)
          return
        }

        const { createSession: { userEdge, sessionEdge } } = payload
        console.log('User fetched from api, saving to session', userEdge, sessionEdge)
        done(null, { id: userEdge.node.id, sessionId: sessionEdge.node.id })
      },

      onError(error) {
        console.log('Something happened:', error, error.source.errors)
        done(null, null)
      },
    })
  }

  app.use(session({
    store: new RedisStore({ host: process.env.DIGRAPH_REDIS_HOST || '' }),
    secret: process.env.DIGRAPH_COOKIE_SECRET || 'keyboard cat',
    resave: true,
    saveUninitialized: true,
  }))

  app
    .use(passport.initialize())
    .use(passport.session())
    .use(bodyParser.urlencoded({ extended: true }))
    .use(cookieParser())

  app.get('/auth/github', passport.authenticate('github'))

  app.get(
    '/auth/github/callback',
    passport.authenticate('github', { failureRedirect: '/login' }),
    async (req, res) => {
      console.log('GitHub auth succeeded, redirecting to /')
      res.redirect('/')
    },
  )

  app.get('/logout', (req, res) => {
    deleteSessionMutation(
      environment,
      [],
      { sessionId: req.user.sessionId },
      {
        onCompleted() {
          console.log('Deleted session for user', req.user.id)
          req.logout()
          res.redirect('/')
        },

        onError(error) {
          console.log(`Failed to delete session for user ${req.user.id}`, error, error.source.errors)
        },
      },
    )
  })

  passport.use(
    new Strategy(
      {
        clientID: process.env.DIGRAPH_GITHUB_CLIENT_ID || 'GitHub client id needed',
        clientSecret: process.env.DIGRAPH_GITHUB_CLIENT_SECRET || 'GitHub client secret needed',
        callbackURL: '/auth/github/callback',
        scope: 'user:email',
      },
      onGithubAuthSuccess,
    ),
  )

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
