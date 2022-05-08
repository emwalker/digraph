import React, { useCallback, useState, FormEvent, ChangeEvent, KeyboardEvent } from 'react'
import { Router } from 'found'
import classNames from 'classnames'
import { EditorState, DraftHandleValue } from 'draft-js'

import { everythingTopicPath } from 'components/constants'
import { LocationType } from 'components/types'
import { SearchBox_view as ViewType } from '__generated__/SearchBox_view.graphql'
import TextInput from './TextInput'
import styles from './styles.module.css'
import queryFromState from './queryFromState'

type Props = {
  className?: string,
  location: LocationType,
  router: Router,
  showButton?: boolean,
  view: ViewType,
}

const atHomepage = (pathname: string) => pathname === '/'

const pathnameFor = (pathname: string, selectedScope: string) => {
  if (atHomepage(pathname)) return everythingTopicPath
  return selectedScope === 'Everything' ? everythingTopicPath : pathname
}

const onFormSubmit = (event: FormEvent<HTMLFormElement>) => {
  event.preventDefault()
}

const inputSelect = (
  pathname: string,
  selectedScope: string,
  onSelectChange: (event: ChangeEvent<HTMLSelectElement>) => void,
) => {
  if (atHomepage(pathname)) {
    return (
      <button className={classNames('btn', styles.button)} type="button">
        Everything
      </button>
    )
  }

  return (
    <select
      onChange={onSelectChange}
      value={selectedScope}
      className={classNames('btn', styles.button)}
    >
      <option>Everything</option>
      <option>This topic</option>
    </select>
  )
}

const SearchBoxInner = ({ className, router, location, showButton, view }: Props) => {
  const { pathname } = location
  const [selectedScope, setSelectedScope] = useState('Everything')

  const handleReturn = useCallback(
    (event: KeyboardEvent<Element>, editorState: EditorState): DraftHandleValue => {
      const query = queryFromState(editorState).toString()
      const searchPathname = pathnameFor(pathname, selectedScope)

      if (query === '') {
        router.push({ pathname: searchPathname })
        return 'handled'
      }

      router.push({ pathname: searchPathname, query: { q: query } })
      return 'handled'
    }, [router, pathname, selectedScope],
  )

  const onSelectChange = useCallback((event: FormEvent<HTMLSelectElement>) => {
    const { value } = event.currentTarget
    setSelectedScope(value)
  }, [setSelectedScope])

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
