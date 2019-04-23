// @flow
import React, { Component } from 'react'

import GithubLogin from 'components/ui/GithubLogin'

/* eslint jsx-a11y/label-has-for: 0 */

type Props = {}

class SignInPage extends Component<Props> {
  renderSignInButton = () => (
    <button
      disabled={false}
      className="btn btn-primary"
    >
      Sign in
    </button>
  )

  render = () => (
    <div className="SignInPage clearfix">
      <div className="col-6 column">
        <h2 className="mb-2">Sign in</h2>
        <p className="mb-2">Log in with your GitHub account:</p>

        <GithubLogin className="mb-5">
          Log in with GitHub
        </GithubLogin>
      </div>
    </div>
  )
}

export default SignInPage
