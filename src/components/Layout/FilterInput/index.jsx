// @flow
import React, { Component } from 'react'

type Props = {
  onEnter?: ?Function,
  value: string,
}

type State = {
  value: string,
}

class FilterInput extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      value: props.value,
    }
  }

  componentWillReceiveProps(nextProps) {
    this.setState({ value: nextProps.value })
  }

  onChange = (event) => {
    this.setState({ value: event.target.value })
  }

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
      className="form-control"
      onChange={this.onChange}
      onKeyPress={this.onKeyPress}
      placeholder="Search"
      size="40"
      type="text"
      value={this.state.value}
    />
  )
}

export default FilterInput
