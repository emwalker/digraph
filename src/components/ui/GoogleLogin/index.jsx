// @flow
import React, { useCallback } from 'react'
import { GoogleLoginButton } from 'react-social-login-buttons'

type Props = {
  className?: ?string,
}

const GoogleLogin = ({ className }: Props) => {
  const onClick = useCallback(() => {
    window.location.href = '/auth/google'
  }, [])

  return (
    <GoogleLoginButton className={className} onClick={onClick}>
      Log in with Google
    </GoogleLoginButton>
  )
}

GoogleLogin.defaultProps = {
  className: null,
}

export default GoogleLogin
