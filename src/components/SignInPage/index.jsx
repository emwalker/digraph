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
    <div className="SignInPage">
      <div className="one-half column">
        <h2 className="mb-2">Sign in</h2>
        <p className="mb-2">Log in with your GitHub account:</p>

        <GithubLogin className="mb-5">
          Log in with GitHub
        </GithubLogin>

        <p className="mb-2">Or sign in with your email address and password:</p>

        <form>
          <dl className="form-group required">
            <dt>
              <label htmlFor="email">Email address</label>
            </dt>
            <dd>
              <input id="email" className="form-control" type="text" />
            </dd>
          </dl>
          <dl className="form-group required">
            <dt>
              <label htmlFor="password">Password</label>
            </dt>
            <dd>
              <input id="password" className="form-control" type="password" />
            </dd>
          </dl>
        </form>

        {this.renderSignInButton()}
      </div>
    </div>
  )
}

export default SignInPage
