// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Link } from 'found'

import type { UserType } from 'components/types'
import DigraphLogo from 'components/ui/icons/DigraphLogo'
import Menu from './Menu'

type Props = {
  viewer: UserType,
}

type State = {
  isOpen: boolean,
}

class MobileNav extends Component<Props, State> {
  state = {
    isOpen: false,
  }

  onClick = () => {
    this.setState(prevState => ({ isOpen: !prevState.isOpen }))
  }

  render = () => (
    <div className="mobile-menu mb-4">
      <div className="mobile-menu-header px-3 py-2">
        <Link
          to="/"
          className="h3 text-normal menu-logo text-gray-dark n-link no-underline d-inline-block"
        >
          <DigraphLogo width="32px" height="32px" />
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
    }
  `,
})
