// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Link } from 'found'
import classNames from 'classnames'

import DigraphLogo from 'components/ui/icons/DigraphLogo'
import Menu from './Menu'
import styles from './styles.module.css'
import type { MobileNav_viewer as Viewer } from './__generated__/MobileNav_viewer.graphql'

type Props = {
  viewer: Viewer,
}

type State = {
  isOpen: boolean,
}

class MobileNav extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      isOpen: false,
    }
  }

  onClick = () => {
    this.setState((prevState) => ({ isOpen: !prevState.isOpen }))
  }

  render = () => (
    <div className={classNames(styles.mobileMenu, 'mobile-menu mb-4')}>
      <div className="mobile-menu-header d-flex px-3 py-2">
        <Link
          to="/"
          className={classNames(
            styles.link,
            'h3 text-normal menu-logo text-gray-dark n-link no-underline d-flex flex-items-center',
          )}
        >
          <div className="mr-2 d-inline-block">
            <DigraphLogo width="32px" height="32px" />
          </div>
          {' '}
          Digraph
        </Link>

        <button
          className="menu-btn btn btn-outline py-1"
          onClick={this.onClick}
          type="button"
        >
          Menu
        </button>
      </div>
      { this.state.isOpen && <Menu viewer={this.props.viewer} /> }
    </div>
  )
}

export const UnwrappedMobileNav = MobileNav

export default createFragmentContainer(MobileNav, {
  viewer: graphql`
    fragment MobileNav_viewer on User {
      isGuest
      ...Menu_viewer
    }
  `,
})
