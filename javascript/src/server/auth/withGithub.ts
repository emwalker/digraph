import { Express } from 'express'
import passport from 'passport'
import { Strategy, Profile } from 'passport-github2'
import { Environment } from 'relay-runtime'

import createGithubSessionMutation, { Input, Response } from 'mutations/createGithubSessionMutation'
import registerEndpointsFn from './registerEndpointsFn'

/* eslint no-console: 0 */

interface IProfile extends Profile {
  _json: {
    avatar_url: string
  }
}

type Email = {
  primary?: boolean,
  value: string
}

type Done = (err?: Error | null, profile?: any) => void

const primaryOrFirstEmail = (emails: Email[]) => {
  const matches = emails.filter(({ primary }) => primary)
  if (matches.length) return matches[0].value
  if (emails.length) return emails[0].value
  return null
}

const onAuthSuccessFn = (environment: Environment) => async (
  _arg0: any, _arg1: any, profile: IProfile, done: Done,
) => {
  console.log('GitHub login succeeded, getting viewer id')

  // eslint-disable-next-line @typescript-eslint/naming-convention
  const { displayName, emails, username, _json: { avatar_url } } = profile
  const email = primaryOrFirstEmail(emails || [])
  console.log(`User ${email || '(no email)'} logging in`)
  if (!email || !username) return

  const input: Input = {
    githubAvatarUrl: avatar_url,
    githubUsername: username,
    name: displayName || 'Nemo',
    primaryEmail: email,
    serverSecret: process.env.DIGRAPH_SERVER_SECRET || 'keyboard cat',
  }

  createGithubSessionMutation(environment, input, {
    onCompleted(payload: Response) {
      if (!payload.createGithubSession) {
        console.log('createGithubSession field missing from response:', payload)
        done(null, null)
        return
      }

      const { createGithubSession } = payload
      const userEdge = createGithubSession?.userEdge
      const sessionEdge = createGithubSession?.sessionEdge
      console.log('User fetched from api, saving to session', userEdge, sessionEdge)
      const id = userEdge && userEdge.node && userEdge.node.id
      const sessionId = sessionEdge?.node?.id
      done(null, { id, sessionId })
    },

    onError(error: Error) {
      console.log('Something happened:', error)
      done(null, null)
    },
  })
}

const registerEndpoints = registerEndpointsFn('github')

export default (app: Express, environment: Environment) => {
  registerEndpoints(app)

  passport.use(
    new Strategy(
      {
        clientID: process.env.DIGRAPH_GITHUB_CLIENT_ID || 'GitHub client id needed',
        clientSecret: process.env.DIGRAPH_GITHUB_CLIENT_SECRET || 'GitHub client secret needed',
        callbackURL: process.env.DIGRAPH_GITHUB_CALLBACK_URL || '',
        scope: ['user:email'],
      },
      onAuthSuccessFn(environment),
    ),
  )
}
