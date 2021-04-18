// @flow
import React from 'react'
import classNames from 'classnames'
import { Link } from 'found'

const className = classNames(
  'SignUp',
  'px-3',
  'Link--primary',
  'HeaderMenu-link',
  'd-inline-block',
  'no-underline',
  'border',
  'rounded-1',
  'px-2',
  'py-1',
)

const SignUp = () => (
  <Link
    to="/join"
    className={className}
  >
    Sign up
  </Link>
)

export default SignUp
