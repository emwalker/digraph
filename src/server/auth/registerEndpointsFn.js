// @flow
import passport from 'passport'
import type { App } from '../types'

const registerEndpointsFn = (provider: string) => (app: App) => {
  app.get(`/auth/${provider}`, passport.authenticate(provider))

  app.get(
    `/auth/${provider}/callback`,
    passport.authenticate(provider, { failureRedirect: '/login' }),
    async (req, res) => {
      // eslint-disable-next-line no-console
      console.log(`Auth with ${provider} succeeded, redirecting to /`)
      res.redirect('/')
    },
  )

  return app
}

export default registerEndpointsFn
