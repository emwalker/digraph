// @flow
import React from 'react'
import type { Node } from 'react'
import DocumentTitle from 'react-document-title'

type Props = {|
  children: Node | string,
  totalCount: number,
|}

const title = 'Links to review'

export default ({ children, totalCount }: Props) => (
  <DocumentTitle title={`${title} | Digraph`}>
    <div className="px-3 px-md-6 px-lg-0">
      <div className="Subhead clearfix gutter">
        <div className="Subhead-heading col-lg-8 col-12">
          { title }
        </div>
      </div>
      <div className="Box">
        <div className="Box-header">
          <h3 className="Box-title overflow-hidden flex-auto">
            Links
            {' '}
            <span className="Counter Counter--light-gray">{ totalCount }</span>
          </h3>
        </div>

        <ul>
          { children }
        </ul>
      </div>
    </div>
  </DocumentTitle>
)
