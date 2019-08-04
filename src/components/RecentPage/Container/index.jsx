// @flow
import React, { Component } from 'react'
import type { Node } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { LineItems_topic as Topic } from './__generated__/Container_topic.graphql'

type Props = {|
  children: Node | string,
  topic: Topic,
|}

class Container extends Component<Props> {
  get title(): string {
    return this.props.topic
      ? `Recent activity within ${this.props.topic.displayName}`
      : 'Recent activity'
  }

  render = () => (
    <div className="px-3 px-md-6 px-lg-0">
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
    </div>
  )
}

export default createFragmentContainer(Container, {
  topic: graphql`
    fragment Container_topic on Topic {
      displayName
    }
  `,
})
