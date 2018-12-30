// @flow
import React, { Component } from 'react'
import classNames from 'classnames'
import Octicon from 'react-component-octicons'

// https://medium.com/@veelenga/displaying-rails-flash-messages-with-react-5f82982f241c

type Props = {
  message: Object,
  onClose: ?Function,
}

class Alert extends Component<Props> {
  get className(): string {
    return classNames(
      'flash fade in mt-3 mb-3',
      this.alertClass(this.props.message.type),
    )
  }

  alertClass = (type: string) => {
    const classes = {
      ERROR: 'flash-error',
      WARN: 'flash-warn',
      SUCCESS: 'flash-success',
    }
    return classes[type] || 'flash-success'
  }

  render = () => (
    <div className={this.className}>
      <button
        className="flash-close"
        onClick={this.props.onClose}
      >
        <Octicon name="x" />
      </button>
      { this.props.message.text }
    </div>
  )
}

export default Alert
