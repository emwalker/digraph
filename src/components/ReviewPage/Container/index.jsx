// @flow
import React from 'react'
import type { Node } from 'react'
import DocumentTitle from 'react-document-title'

type Props = {|
  children: Node | string,
|}

const title = 'Links to review'

export default ({ children }: Props) => (
  <DocumentTitle title={`${title} | Digraph`}>
    <div className="px-3 px-md-6 px-lg-0">
      <div className="Subhead clearfix gutter">
        <div className="Subhead-heading col-lg-8 col-12">
          { title }
        </div>
      </div>
      { children }
    </div>
  </DocumentTitle>
)
