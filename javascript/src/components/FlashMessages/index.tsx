import React, { Component } from 'react'
import update from 'immutability-helper'

import { AlertType } from 'components/types'

// https://medium.com/@veelenga/displaying-rails-flash-messages-with-react-5f82982f241c

import Alert from './Alert'

type AlertPayload = {
  id: string,
  type: AlertType,
  text: string,
}

type Props = {
  initialAlerts: readonly AlertPayload[],
}

type State = {
  messages: readonly AlertPayload[],
}

class FlashMessages extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      messages: props.initialAlerts || [],
    }

    window.flashMessages = this
  }

  get alerts() {
    return this.state.messages.map((message) => (
      <Alert
        key={message.id}
        message={message}
        onClose={() => this.removeMessage(message)}
      />
    ))
  }

  removeMessage = (message: AlertPayload) => {
    const index = this.state.messages.indexOf(message)
    this.setState((prevState) => ({
      messages: update(prevState.messages, { $splice: [[index, 1]] }),
    }))
  }

  addMessage = (message: AlertPayload) => {
    this.setState((prevState) => ({
      messages: update(prevState.messages, { $push: [message] }),
    }))
  }

  render = () => {
    const { alerts } = this

    if (alerts.length === 0) return null

    return (
      <div
        data-testid="alerts"
        className="container-lg clearfix my-2 px-3 px-md-6 px-lg-3"
      >
        { alerts }
      </div>
    )
  }
}

export default FlashMessages
