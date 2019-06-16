// @flow
import React, { Component, type Node } from 'react'
import { GoTrashcan } from 'react-icons/go'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Synonym_synonym as SynonymType } from './__generated__/Synonym_synonym.graphql'

type Props = {
  dragHandle: ?Node,
  index: number,
  onDelete?: ?Function,
  // $FlowFixMe
  synonym: SynonymType,
}

class Synonym extends Component<Props> {
  static defaultProps = {
    onDelete: null,
  }

  onClick = () => {
    if (!this.props.onDelete) return
    this.props.onDelete(this.props.index)
  }

  renderDeleteButton = () => (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events
    <span tabIndex="0" role="button" onClick={this.onClick}>
      <GoTrashcan />
    </span>
  )

  render = () => (
    <li className="Box-row clearfix css-truncate p-2 d-flex">
      { this.props.dragHandle }
      <div className="col-10">{this.props.synonym.name}</div>
      <div className="col-1">
        { this.props.synonym.locale }
      </div>
      <div className="col-1 remove-synonym">
        { this.props.onDelete && this.renderDeleteButton() }
      </div>
    </li>
  )
}

export const UnwrappedSynonym = Synonym

export default createFragmentContainer(Synonym, {
  synonym: graphql`
    fragment Synonym_synonym on Synonym {
      name
      locale
    }
  `,
})
