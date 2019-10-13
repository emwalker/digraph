// @flow
import React, { Component } from 'react'
import classNames from 'classnames'

type Props = {
  className?: ?string,
  onEnter?: ?Function,
  value: string,
}

type State = {
  value: string,
}

class SearchBox extends Component<Props, State> {
  static defaultProps = {
    className: '',
    onEnter: null,
  }

  constructor(props: Props) {
    super(props)
    this.state = {
      value: props.value,
    }
  }

  // eslint-disable-next-line camelcase
  UNSAFE_componentWillReceiveProps(nextProps: Props) {
    this.setState({ value: nextProps.value })
  }

  onChange = (event: SyntheticEvent<HTMLButtonElement>) => {
    const { value } = (event.target: window.HTMLInputElement)
    this.setState({ value })
  }

  onEnter = (string: string) => {
    if (this.props.onEnter) this.props.onEnter(string)
  }

  onKeyPress = (event: SyntheticKeyboardEvent<HTMLButtonElement>) => {
    if (event.key === 'Enter') {
      const { value } = (event.target: window.HTMLInputElement)
      this.onEnter(value)
    }
  }

  get className(): string {
    return classNames('SearchBox form-group mb-1 mt-1', this.props.className)
  }

  render = () => (
    <div className={this.className}>
      <input
        aria-label="Search"
        className="form-control"
        onChange={this.onChange}
        onKeyPress={this.onKeyPress}
        placeholder="Search"
        type="search"
        value={this.state.value}
      />
    </div>
  )
}

export default SearchBox
