// @flow
import React, { useCallback } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import { pathOr } from 'ramda'

import useDocumentTitle from 'utils/useDocumentTitle'
import type { ViewType } from 'components/types'
import SearchBox from 'components/ui/SearchBox'
import './styles.module.css'

const resourcePath = pathOr('/', ['currentRepository', 'rootTopic', 'resourcePath'])

type Props = {
  heading: string,
  location: {
    pathname: string,
    query: Object,
    search: string,
  },
  renderHeadingDetail?: Function,
  router: {
    push: Function,
  },
  view: ViewType,
}

const Subhead = (props: Props) => {
  const { heading, location, renderHeadingDetail, router, view } = props
  const pathname = resourcePath(view)

  const onSearch = useCallback((query: string) => {
    if (query === '') {
      router.push({ pathname })
      return
    }

    router.push({ pathname, query: { q: query } })
  }, [router, pathname])

  const searchString = location.search
    ? location.query.q
    : ''

  useDocumentTitle(`${heading} | Digraph`)

  return (
    <div className="Subhead clearfix gutter">
      <div className="Subhead-heading col-lg-8 col-12 d-inline-flex">
        { renderHeadingDetail && renderHeadingDetail() }
        <div>{ heading }</div>
      </div>
      <SearchBox
        className="col-lg-4 col-12"
        onEnter={onSearch}
        value={searchString}
      />
    </div>
  )
}

Subhead.defaultProps = {
  renderHeadingDetail: null,
}

export default createFragmentContainer(Subhead, {
  view: graphql`
    fragment Subhead_view on View {
      currentRepository {
        rootTopic {
          resourcePath
        }
      }
    }
  `,
})
