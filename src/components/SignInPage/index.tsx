import React, { Component } from 'react'

import Page from 'components/ui/Page'
import GithubLogin from 'components/ui/GithubLogin'

type Props = {}

class SignInPage extends Component<Props> {
  renderSignInButton = () => (
    <button
      className="btn btn-primary"
      disabled={false}
      type="button"
    >
      Sign in
    </button>
  )

  render = () => (
    <Page>
      <div className="SignInPage clearfix">
        <div className="col-10 col-md-6 column">
          <h2 className="mb-2">Sign in</h2>
          <p className="mb-2">Log in with your GitHub account:</p>
          <GithubLogin className="mb-5" />
        </div>
      </div>
    </Page>
  )
}

export default SignInPage
