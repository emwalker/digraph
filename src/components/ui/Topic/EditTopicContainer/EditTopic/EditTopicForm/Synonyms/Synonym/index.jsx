// @flow
import React, { Component } from 'react'
import Octicon from 'react-component-octicons'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Synonym_synonym as SynonymType } from './__generated__/Synonym_synonym.graphql'

type Props = {
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
    this.props.onDelete(this.props.synonym)
  }

  renderDeleteButton = () => (
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events
    <span tabIndex="0" role="button" onClick={this.onClick}>
      <Octicon name="trashcan" />
    </span>
  )

  render = () => (
    <li className="Box-row clearfix css-truncate p-2">
      <div className="col-10 float-left">{this.props.synonym.name}</div>
      <div className="col-1 float-right remove-synonym">
        { this.props.onDelete && this.renderDeleteButton() }
      </div>
      <div className="col-1 float-right">
        { this.props.synonym.locale }
      </div>
    </li>
  )
}

export const UnwrappedSynonym = Synonym

export default createFragmentContainer(Synonym, {
  synonym: graphql`
    fragment Synonym_synonym on Synonym {
      id
      name
      locale
    }
  `,
})
