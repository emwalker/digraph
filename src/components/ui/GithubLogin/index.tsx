import React, { Component } from 'react'
import { GithubLoginButton } from 'react-social-login-buttons'
import classNames from 'classnames'

type Props = {
  className?: string | undefined,
}

class GithubLogin extends Component<Props> {
  static defaultProps = {
    className: '',
  }

  onClick = () => {
    window.location.href = '/auth/github'
  }

  render = () => (
    <div className={classNames('GithubLogin', this.props.className)}>
      <GithubLoginButton {...this.props} onClick={this.onClick}>
        Log in with GitHub
      </GithubLoginButton>
    </div>
  )
}

export default GithubLogin
