import React, { Component, ReactNode } from 'react'

type Props = {
  children: ReactNode,
  topicName?: string,
}

class Container extends Component<Props> {
  get title(): string {
    return this.props.topicName
      ? `Recent activity within ${this.props.topicName}`
      : 'Recent activity'
  }

  render = () => (
    <>
      <div className="Subhead clearfix gutter">
        <div className="Subhead-heading col-lg-8 col-12">
          { this.title }
        </div>
      </div>
      <div className="Box Box--condensed">
        <div className="Box-header">
          <h3 className="Box-title overflow-hidden flex-auto">
            Activity
          </h3>
        </div>
        { this.props.children }
      </div>
    </>
  )
}

export default Container
