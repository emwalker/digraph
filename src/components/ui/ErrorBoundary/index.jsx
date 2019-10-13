// @flow
import React, { Component, type Node } from 'react'

type Props = {
  children: Node,
}

type State = {
  hasError: boolean,
}

class ErrorBoundary extends Component<Props, State> {
  static getDerivedStateFromError() {
    return {
      hasError: true,
    }
  }

  constructor(props: Props) {
    super(props)
    this.state = {
      hasError: false,
    }
  }

  // eslint-disable-next-line class-methods-use-this
  componentDidCatch(error: Error, info: Object) {
    // eslint-disable-next-line no-console
    console.log(error, info)
  }

  render = () => (
    this.state.hasError
      ? <p>Something happened.</p>
      : this.props.children
  )
}

export default ErrorBoundary
