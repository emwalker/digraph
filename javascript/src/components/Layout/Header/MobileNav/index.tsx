import React, { useState, useCallback } from 'react'
import { graphql, useFragment } from 'react-relay'
import { Link, Router } from 'found'
import classNames from 'classnames'

import { LocationType } from 'components/types'
import DigraphLogo from 'components/ui/icons/DigraphLogo'
import SearchBox from 'components/ui/SearchBox'
import { MobileNav_viewer$key } from '__generated__/MobileNav_viewer.graphql'
import Menu from './Menu'
import styles from './styles.module.css'

type Props = {
  location: LocationType,
  router: Router,
  showButton?: boolean,
  viewer: MobileNav_viewer$key,
}

export default function MobileNav(props: Props) {
  const viewer = useFragment(
    graphql`
      fragment MobileNav_viewer on User {
        isGuest
        ...Menu_viewer
      }
    `,
    props.viewer,
  )

  const [isOpen, setIsOpen] = useState(false)
  
  const onClick = useCallback(() => {
    setIsOpen(!isOpen)
  }, [setIsOpen])

  return (
    <div className={classNames(styles.mobileMenu, 'mobile-menu')}>
      <div className={classNames(styles.mobileMenuHeader, 'mobile-menu-header d-flex px-3 py-2')}>
        <div className={styles.logo}>
          <Link
            to="/"
            className={classNames(
              styles.link,
              'menu-logo Link--primary n-link no-underline d-flex flex-items-center',
            )}
          >
            <div className="mr-1 d-inline-block">
              <DigraphLogo width="32px" height="32px" fill="#000" />
            </div>
            {' '}
            Digraph
          </Link>
        </div>

        <div className={styles.searchBox}>
          <SearchBox
            location={props.location}
            router={props.router}
            showButton={props.showButton}
          />
        </div>

        <div className={styles.rightButton}>
          <button
            className="menu-btn btn btn-outline py-1"
            onClick={onClick}
            type="button"
          >
            Menu
          </button>
        </div>
      </div>

      {isOpen && <Menu viewer={viewer} />}
    </div>
  )
}
