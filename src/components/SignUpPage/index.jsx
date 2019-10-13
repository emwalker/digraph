// @flow
import React, { Component } from 'react'
import ReCAPTCHA from 'react-google-recaptcha'

import GithubLogin from 'components/ui/GithubLogin'

/* eslint react/no-string-refs: 0 */

type State = {
  gCaptchaResponse: ?string,
}

type Props = {}

// http://emumba.com/blog/2016-12-07-setting-up-google-recaptcha-in-a-reactjs-app/

class SignUpPage extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      gCaptchaResponse: null,
    }
  }

  onCaptchaSumit = (response: string) => {
    this.setState({ gCaptchaResponse: response })
  }

  renderCreateAccountButton = () => (
    <button
      disabled={!this.state.gCaptchaResponse}
      className="btn btn-primary"
      type="submit"
    >
      Create an account
    </button>
  )

  render = () => (
    <div className="SignUpPage clearfix">
      <div className="col-6">
        <h1>Join Digraph</h1>
        <p className="lead">
          {'Keep track of everything you\'ve ever read on the Internet.'}
        </p>

        <GithubLogin className="mb-5">
          Log in with GitHub
        </GithubLogin>

        <h2 className="f2-light mb-1">Or create an account</h2>

        <form>
          <dl className="form-group required">
            <dt><label htmlFor="username">Username</label></dt>
            <dd>
              <input id="username" className="form-control" type="text" />
              <p className="note">Used similarly to a GitHub username.</p>
            </dd>
          </dl>
          <dl className="form-group required">
            <dt><label htmlFor="email">Email address</label></dt>
            <dd>
              <input id="email" className="form-control" type="text" />
              <p className="note">
                For getting in touch with you if something important comes up. We will not
                sell your email address to third parties.
              </p>
            </dd>
          </dl>
          <dl className="form-group required">
            <dt><label htmlFor="password">Password</label></dt>
            <dd>
              <input id="password" className="form-control" type="text" />
              <p className="note">
                At least 15 characters long.
              </p>
            </dd>
          </dl>
        </form>

        <h2 className="mb-2">Verify account</h2>
        <div className="width-full mb-4">
          <ReCAPTCHA
            ref="recaptcha"
            sitekey="6LfOQosUAAAAAClaTyOmN0d64syvFjxdys1nUkbd"
            onChange={this.onCaptchaSumit}
          />
        </div>

        {this.renderCreateAccountButton()}
      </div>
    </div>
  )
}

export default SignUpPage
