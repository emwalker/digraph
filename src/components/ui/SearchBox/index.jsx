// @flow
import React, { useCallback, useState } from 'react'
import classNames from 'classnames'

import { everythingTopicPath } from 'components/constants'
import type { Location, Router } from 'components/types'
import styles from './styles.module.css'

type Props = {
  className?: string,
  location: Location,
  router: Router,
  showButton?: boolean,
}

const atHomepage = (pathname: string) => pathname === '/'

const pathnameFor = (pathname: string, selectedScope: string) => {
  if (atHomepage(pathname)) return everythingTopicPath
  return selectedScope === 'Everything' ? everythingTopicPath : pathname
}

const onFormSubmit = (event: SyntheticKeyboardEvent<HTMLButtonElement>) => {
  event.preventDefault()
}

const inputSelect = (pathname: string, selectedScope: string, onSelectChange: Function) => {
  if (atHomepage(pathname)) {
    return (
      <button className="btn" type="button">
        Everything
      </button>
    )
  }

  return (
    <select onChange={onSelectChange} value={selectedScope} className="btn">
      <option>Everything</option>
      <option>This topic</option>
    </select>
  )
}

const SearchBox = ({ className, router, location, showButton }: Props) => {
  const { pathname } = location
  const searchString = location.search ? location.query.q : ''
  const [selectedScope, setSelectedScope] = useState('Everything')

  const onKeyPress = useCallback((event: SyntheticKeyboardEvent<HTMLButtonElement>) => {
    if (event.key === 'Enter') {
      const { value } = (event.target: window.HTMLInputElement)
      const searchPathname = pathnameFor(pathname, selectedScope)

      if (value === '') {
        router.push({ pathname: searchPathname })
        return
      }

      router.push({ pathname: searchPathname, query: { q: value } })
    }
  }, [router, pathname, selectedScope])

  const onSelectChange = useCallback((event: SyntheticKeyboardEvent<HTMLButtonElement>) => {
    const { value } = (event.target: window.HTMLInputElement)
    setSelectedScope(value)
  })

  const actualClassName = classNames(styles.searchBox, 'input-group', className)

  return (
    <form className={actualClassName} onSubmit={onFormSubmit}>
      <input
        aria-label="Search"
        className={classNames('form-control', styles.searchInput)}
        onKeyPress={onKeyPress}
        placeholder="Search"
        type="search"
        defaultValue={searchString}
      />
      {showButton && (
        <span className="input-group-button">
          {inputSelect(pathname, selectedScope, onSelectChange)}
        </span>
      )}
    </form>
  )
}

SearchBox.defaultProps = {
  className: '',
  showButton: true,
}

export default SearchBox
