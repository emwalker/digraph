import React, { Component, ReactNode } from 'react'
import { GoTrashcan } from 'react-icons/go'
import { createFragmentContainer, graphql } from 'react-relay'

import { Synonym_synonym as SynonymType } from '__generated__/Synonym_synonym.graphql'

type Props = {
  dragHandle?: ReactNode,
  onDelete?: (index: number) => void,
  position?: number,
  synonym: SynonymType,
}

class Synonym extends Component<Props> {
  static defaultProps = {
    onDelete: undefined,
  }

  onClick = () => {
    if (!this.props.onDelete || !this.props.position) return
    this.props.onDelete(this.props.position)
  }

  renderDeleteButton = () => (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events
    <span tabIndex={0} role="button" onClick={this.onClick}>
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
