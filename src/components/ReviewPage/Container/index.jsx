// @flow
import React, { Component } from 'react'
import type { Node } from 'react'
import DocumentTitle from 'react-document-title'

type Props = {|
  children: Node | string,
  topicName: ?string,
  totalCount: number,
|}

class Container extends Component<Props> {
  get title(): string {
    return this.props.topicName
      ? `Links to review within ${this.props.topicName}`
      : 'Links to review'
  }

  render = () => (
    <DocumentTitle title={`${this.title} | Digraph`}>
      <div className="px-3 px-md-6 px-lg-0">
        <div className="Subhead clearfix gutter">
          <div className="Subhead-heading col-lg-8 col-12">
            { this.title }
          </div>
        </div>
        <div className="Box Box--condensed">
          <div className="Box-header">
            <h3 className="Box-title overflow-hidden flex-auto">
              Links
              {' '}
              <span className="Counter Counter--light-gray">{ this.props.totalCount }</span>
            </h3>
          </div>

          <ul>
            { this.props.children }
          </ul>
        </div>
      </div>
    </DocumentTitle>
  )
}

export default Container
