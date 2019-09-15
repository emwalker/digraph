import passport from 'passport'

const registerEndpointsFn = provider => (app) => {
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
