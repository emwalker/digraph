import React, { Component } from 'react'
import classNames from 'classnames'
import { GoX } from 'react-icons/go'

import { AlertType } from 'components/types'

// https://medium.com/@veelenga/displaying-rails-flash-messages-with-react-5f82982f241c

type Props = {
  message: {
    text: string,
    type: AlertType,
  },
  onClose: () => void,
}

class Alert extends Component<Props> {
  get className(): string {
    return classNames(
      'flash fade in mt-3 mb-3',
      this.alertClass(this.props.message.type),
    )
  }

  alertClass = (type: AlertType) => {
    const classes = {
      ERROR: 'flash-error',
      WARN: 'flash-warn',
      SUCCESS: 'flash-success',
      '%future added value': 'flash-error',
    }
    return classes[type] || 'flash-success'
  }

  render = () => (
    <div className={this.className}>
      <button
        className="flash-close"
        onClick={this.props.onClose}
        type="button"
      >
        <GoX />
      </button>
      { this.props.message.text }
    </div>
  )
}

export default Alert
