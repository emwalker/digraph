// @flow
import React from 'react'
import type { Node } from 'react'

type Props = {|
  children: Node | string,
|}

export default ({ children }: Props) => (
  <div className="px-3 px-md-6 px-lg-0">
    <div className="Subhead clearfix gutter">
      <div className="Subhead-heading col-lg-8 col-12">
        Recent activity
      </div>
    </div>
    <div className="Box Box--condensed">
      <div className="Box-header">
        <h3 className="Box-title overflow-hidden flex-auto">
          Activity
        </h3>
      </div>
      { children }
    </div>
  </div>
)
