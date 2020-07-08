// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { Link } from 'found'
import classNames from 'classnames'

import type { Location, Router } from 'components/types'
import DigraphLogo from 'components/ui/icons/DigraphLogo'
import SearchBox from 'components/ui/SearchBox'
import Menu from './Menu'
import styles from './styles.module.css'
import type { MobileNav_viewer as Viewer } from './__generated__/MobileNav_viewer.graphql'

type Props = {
  location: Location,
  router: Router,
  showButton?: boolean,
  viewer: Viewer,
}

type State = {
  isOpen: boolean,
}

class MobileNav extends Component<Props, State> {
  static defaultProps = {
    showButton: true,
  }

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
    <div className={classNames(styles.mobileMenu, 'mobile-menu')}>
      <div className={classNames(styles.mobileMenuHeader, 'mobile-menu-header d-flex px-3 py-2')}>
        <div className={styles.logo}>
          <Link
            to="/"
            className={classNames(
              styles.link,
              'menu-logo text-gray-dark n-link no-underline d-flex flex-items-center',
            )}
          >
            <div className="mr-1 d-inline-block">
              <DigraphLogo width="32px" height="32px" />
            </div>
            {' '}
            Digraph
          </Link>
        </div>

        <div className={styles.searchBox}>
          <SearchBox
            location={this.props.location}
            router={this.props.router}
            showButton={this.props.showButton}
          />
        </div>

        <div className={styles.rightButton}>
          <button
            className="menu-btn btn btn-outline py-1"
            onClick={this.onClick}
            type="button"
          >
            Menu
          </button>
        </div>
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
