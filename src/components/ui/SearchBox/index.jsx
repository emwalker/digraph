// @flow
import React, { Component } from 'react'

type Props = {
  onEnter?: ?Function,
  value: string,
}

type State = {
  value: string,
}

class SearchBox extends Component<Props, State> {
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
    <div className="form-group mb-1 mt-1" style={{ width: '317px' }}>
      <input
        aria-label="Search"
        className="form-control"
        onChange={this.onChange}
        onKeyPress={this.onKeyPress}
        placeholder="Search"
        size="20"
        type="text"
        value={this.state.value}
      />
    </div>
  )
}

export default SearchBox
