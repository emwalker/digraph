// @flow
import React from 'react'
import type { Node } from 'react'

import useDocumentTitle from 'utils/useDocumentTitle'

type Props = {|
  children: Node | string,
  topicName: ?string,
  totalCount: number,
|}

const Container = (props: Props) => {
  const title = props.topicName
    ? `Links to review within ${props.topicName}`
    : 'Links to review'

  useDocumentTitle(title)

  return (
    <>
      <div className="Subhead clearfix gutter">
        <div className="Subhead-heading col-lg-8 col-12">
          { title }
        </div>
      </div>
      <div className="Box Box--condensed">
        <div className="Box-header">
          <h3 className="Box-title overflow-hidden flex-auto">
            Links
            {' '}
            <span className="Counter Counter--light-gray">{ props.totalCount }</span>
          </h3>
        </div>

        <ul>
          { props.children }
        </ul>
      </div>
    </>
  )
}

export default Container
