// @flow
import React, { Component, type Node } from 'react'

type Props = {
  children: Node,
}

class ErrorBoundary extends Component<Props> {
  static getDerivedStateFromError() {
    return {
      hasError: true,
    }
  }

  state = {
    hasError: false,
  }

  // eslint-disable-next-line class-methods-use-this
  componentDidCatch(error, info) {
    // eslint-disable-next-line no-console
    console.log(error, info)
  }

  render = () => (
    this.state.hasError
      ? <h1>Something went wrong.</h1>
      : this.props.children
  )
}

export default ErrorBoundary
