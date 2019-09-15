import passport from 'passport'
import { Strategy } from 'passport-github'

import createGithubSessionMutation from 'mutations/createGithubSessionMutation'
import registerEndpointsFn from './registerEndpointsFn'

/* eslint no-console: 0 */

const primaryOrFirstEmail = (emails) => {
  const matches = emails.filter(({ primary }) => primary)
  if (matches.length) return matches[0].value
  if (emails.length) return emails[0].value
  return null
}

const onAuthSuccessFn = environment => async (accessToken, refreshToken, profile, done) => {
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

  createGithubSessionMutation(environment, [], input, {
    onCompleted(payload) {
      if (!payload.createGithubSession) {
        console.log('createGithubSession field missing from response:', payload)
        done(null, null)
        return
      }

      const { createGithubSession: { userEdge, sessionEdge } } = payload
      console.log('User fetched from api, saving to session', userEdge, sessionEdge)
      done(null, { id: userEdge.node.id, sessionId: sessionEdge.node.id })
    },

    onError(error) {
      console.log('Something happened:', error, error.source.errors)
      done(null, null)
    },
  })
}

const registerEndpoints = registerEndpointsFn('github')

export default (app, environment) => {
  registerEndpoints(app)

  passport.use(
    new Strategy(
      {
        clientID: process.env.DIGRAPH_GITHUB_CLIENT_ID || 'GitHub client id needed',
        clientSecret: process.env.DIGRAPH_GITHUB_CLIENT_SECRET || 'GitHub client secret needed',
        callbackURL: process.env.DIGRAPH_GITHUB_CALLBACK_URL || 'GitHub callback url needed',
        scope: 'user:email',
      },
      onAuthSuccessFn(environment),
    ),
  )
}
