// @flow
import React, { useCallback, useState } from 'react'
import classNames from 'classnames'
import { EditorState } from 'draft-js'

import { everythingTopicPath } from 'components/constants'
import type { Location, Router } from 'components/types'
import TextInput from './TextInput'
import styles from './styles.module.css'
import queryFromState from './queryFromState'

import { type SearchBox_view as View } from '../__generated__/SearchBox_view.graphql'

export type QueryInfo = $PropertyType<View, 'queryInfo'>

type Props = {
  className?: string,
  location: Location,
  router: Router,
  showButton?: boolean,
  view: View,
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
      <button className={classNames('btn', styles.button)} type="button">
        Everything
      </button>
    )
  }

  return (
    <select onChange={onSelectChange} value={selectedScope} className={classNames('btn', styles.button)}>
      <option>Everything</option>
      <option>This topic</option>
    </select>
  )
}

const SearchBoxInner = ({ className, router, location, showButton, view }: Props) => {
  const { pathname } = location
  const [selectedScope, setSelectedScope] = useState('Everything')

  const handleReturn = useCallback((event, editorState: EditorState) => {
    const query = queryFromState(editorState).toString()
    const searchPathname = pathnameFor(pathname, selectedScope)

    if (query === '') {
      router.push({ pathname: searchPathname })
      return
    }

    router.push({ pathname: searchPathname, query: { q: query } })
  }, [router, pathname, selectedScope])

  const onSelectChange = useCallback((event: SyntheticKeyboardEvent<HTMLButtonElement>) => {
    const { value } = (event.target: window.HTMLInputElement)
    setSelectedScope(value)
  })

  const actualClassName = classNames(styles.searchBox, 'input-group', className)

  return (
    <form className={actualClassName} onSubmit={onFormSubmit}>
      <TextInput
        handleReturn={handleReturn}
        queryInfo={view?.queryInfo}
      />
      {showButton && (
        <div className={classNames('input-group-button', styles.buttonContainer)}>
          {inputSelect(pathname, selectedScope, onSelectChange)}
        </div>
      )}
    </form>
  )
}

SearchBoxInner.defaultProps = {
  className: '',
  showButton: true,
}

export default SearchBoxInner
