import passport from 'passport'
import { Strategy } from 'passport-github'
import session from 'express-session'
import connectRedis from 'connect-redis'
import cookieParser from 'cookie-parser'
import bodyParser from 'body-parser'

import upsertUserMutation from 'mutations/upsertUserMutation'
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
    }

    upsertUserMutation(environment, [], input, {
      onCompleted(payload) {
        if (!payload.upsertUser) {
          console.log('upsertUser field missing from response:', payload)
          done(null, null)
          return
        }

        const { upsertUser: { userEdge } } = payload
        console.log('User fetched from api, saving to session', userEdge)
        done(null, { id: userEdge.node.id })
      },
      onError(error) {
        console.log('Something happened:', error, error.source.errors)
        done(null, null)
      },
    })
  }

  app.use(session({
    store: new RedisStore(),
    secret: 'keyboard cat',
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

  passport.use(
    new Strategy(
      {
        clientID: process.env.DIGRAPH_GITHUB_CLIENT_ID,
        clientSecret: process.env.DIGRAPH_GITHUB_CLIENT_SECRET,
        callbackURL: 'http://localhost:3001/auth/github/callback',
        scope: 'user:email',
      },
      onGithubAuthSuccess,
    ),
  )

  passport.serializeUser((viewer, done) => {
    console.log('serializeUser', viewer)
    done(null, viewer.id)
  })

  passport.deserializeUser((id, done) => {
    console.log('deserializeUser', id)
    done(null, { id })
  })

  return app
}
