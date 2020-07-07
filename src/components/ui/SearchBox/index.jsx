// @flow
import React, { useCallback } from 'react'
import classNames from 'classnames'

import { everythingTopicPath } from 'components/constants'
import type { Location, Router } from 'components/types'
import styles from './styles.module.css'

type Props = {
  className?: string,
  location: Location,
  router: Router,
}

const pathnameFor = (pathname: string) => (
  pathname === '/' ? everythingTopicPath : pathname
)

const onFormSubmit = (event: SyntheticKeyboardEvent<HTMLButtonElement>) => {
  event.preventDefault()
}

const SearchBox = ({ className, router, location }: Props) => {
  const { pathname } = location

  const searchString = location.search
    ? location.query.q
    : ''

  const scopeLabel = pathname === '/' ? 'Everything' : 'This topic'

  const onKeyPress = useCallback((event: SyntheticKeyboardEvent<HTMLButtonElement>) => {
    if (event.key === 'Enter') {
      const { value } = (event.target: window.HTMLInputElement)
      const newPathname = pathnameFor(pathname)

      if (value === '') {
        router.push({ pathname: newPathname })
        return
      }

      router.push({ pathname: newPathname, query: { q: value } })
    }
  }, [router, pathname])

  const actualClassName = classNames(styles.searchBox, 'input-group', className)

  return (
    <form className={actualClassName} onSubmit={onFormSubmit}>
      <span className="input-group-button">
        <button className="btn" type="button">{scopeLabel}</button>
      </span>
      <input
        aria-label="Search"
        className={classNames('form-control', styles.searchInput)}
        onKeyPress={onKeyPress}
        placeholder="Search"
        type="search"
        defaultValue={searchString}
      />
    </form>
  )
}

SearchBox.defaultProps = {
  className: '',
}

export default SearchBox
