import React, { useCallback } from 'react'
import { GoogleLoginButton } from 'react-social-login-buttons'

type Props = {
  className?: string | undefined,
}

const GoogleLogin = ({ className }: Props) => {
  const onClick = useCallback(() => {
    window.location.href = '/auth/google'
  }, [])

  return (
    <GoogleLoginButton className={className} onClick={onClick} text="Log in with Google" />
  )
}

GoogleLogin.defaultProps = {
  className: null,
}

export default GoogleLogin
