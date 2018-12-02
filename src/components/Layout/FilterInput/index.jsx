// @flow
import React, { Component } from 'react'

type Props = {
  onEnter?: ?Function,
  value: string,
}

class FilterInput extends Component<Props> {
  onEnter = (string) => {
    if (this.props.onEnter)
      this.props.onEnter(string)
  }

  onKeyPress = (event) => {
    if (event.key === 'Enter')
      this.onEnter(event.target.value)
  }

  render = () => (
    <input
      aria-label="Filter input"
      className="form-control float-right mt-3"
      defaultValue={this.props.value}
      onKeyPress={this.onKeyPress}
      placeholder="Filter"
      size="40"
      type="text"
    />
  )
}

export default FilterInput
