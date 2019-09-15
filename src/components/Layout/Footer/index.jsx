// @flow
import React from 'react'
import { Link } from 'found'

import styles from './styles.module.css'

const Footer = () => (
  <footer className={styles.footer}>
    <div className="container-lg clearfix my-3 px-3 px-md-6 px-lg-3 my-6 pt-2 border-top">
      <p className="mb-2">
        <Link
          className="gray-link"
          to="/terms-of-use"
          test-id="terms-of-use"
        >
          Terms
        </Link>
        {' ・ '}
        Software available under the MIT
        {' '}
        <a
          className="gray-link"
          href="https://github.com/emwalker/digraph/blob/master/LICENSE.md"
        >
          license
        </a>
        . © Eric Walker.
      </p>
    </div>
  </footer>
)

export default Footer
