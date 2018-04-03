import React from 'react'
import Helmet from 'react-helmet'
import { IndexLink } from 'react-router'
import { notFound } from './styles.css'
import { link } from '../homepage/styles.css'

export default () => (
  <div>
    <Helmet title="404 Page Not Found" />
    <h2 className={notFound}>
    404 Page Not Found
    </h2>
    <IndexLink to="/" className={link}>go home</IndexLink>
  </div>
)
