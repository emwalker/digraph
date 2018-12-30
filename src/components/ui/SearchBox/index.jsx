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

  componentWillReceiveProps(nextProps: Props) {
    this.setState({ value: nextProps.value })
  }

  onChange = (event: SyntheticEvent<HTMLButtonElement>) => {
    const { value } = (event.target: window.HTMLInputElement)
    this.setState({ value })
  }

  onEnter = (string: string) => {
    if (this.props.onEnter)
      this.props.onEnter(string)
  }

  onKeyPress = (event: SyntheticKeyboardEvent<HTMLButtonElement>) => {
    if (event.key === 'Enter') {
      const { value } = (event.target: window.HTMLInputElement)
      this.onEnter(value)
    }
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
