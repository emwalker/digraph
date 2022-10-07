import React, { useCallback, useState, FormEvent, ChangeEvent, KeyboardEvent } from 'react'
import { graphql, useFragment } from 'react-relay'
import { Router } from 'found'
import classNames from 'classnames'
import { EditorState, DraftHandleValue } from 'draft-js'

import { topicPath } from 'components/helpers'
import { everythingTopicId } from 'components/constants'
import { LocationType } from 'components/types'
import { SearchBox_view$key } from '__generated__/SearchBox_view.graphql'
import TextInput from './TextInput'
import queryFromState from './queryFromState'

const atHomepage = (pathname: string) => pathname === '/'

const pathnameFor = (pathname: string, selectedScope: string) => {
  if (atHomepage(pathname)) return topicPath(everythingTopicId)
  return selectedScope === 'Everything' ? topicPath(everythingTopicId) : pathname
}

const onFormSubmit = (event: FormEvent<HTMLFormElement>) => {
  event.preventDefault()
}

type SelectScopeProps = {
  pathname: string,
  selectedScope: string,
  onChange: (event: ChangeEvent<HTMLSelectElement>) => void,
}

function SelectScope({ pathname, selectedScope, onChange }: SelectScopeProps) {
  if (atHomepage(pathname)) {
    return (
      <button className="btn searchBoxInnerButton" type="button">
        Everything
      </button>
    )
  }

  return (
    <select
      onChange={onChange}
      value={selectedScope}
      className="btn searchBoxInnerButton"
    >
      <option>Everything</option>
      <option>This topic</option>
    </select>
  )
}

type Props = {
  className?: string,
  location: LocationType,
  router: Router,
  showButton?: boolean,
  view: SearchBox_view$key,
}

export default function SearchBox({ router, className, showButton, location, ...rest }: Props) {
  const view = useFragment(
    graphql`
      fragment SearchBox_view on View {
        queryInfo {
          stringTokens

          topics {
            edges {
              node {
                displayName
                id
              }
            }
          }
        }
      }
    `,
    rest.view,
  )

  const pathname = location.pathname
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

  const actualClassName = classNames('searchBoxInnerSearchBox input-group px-3', className)

  return (
    <form className={actualClassName} onSubmit={onFormSubmit}>
      <TextInput
        handleReturn={handleReturn}
        queryInfo={view?.queryInfo}
      />
      <div className="input-group-button searchBoxInnerButtonContainer">
        <SelectScope
          pathname={pathname}
          selectedScope={selectedScope}
          onChange={onSelectChange}
        />
      </div>
    </form>
  )
}
