import React, { Component, ReactElement } from 'react'
import update from 'immutability-helper'

import { AlertMessageType } from 'components/types'

// https://medium.com/@veelenga/displaying-rails-flash-messages-with-react-5f82982f241c

import Alert from './Alert'

type Props = {
  initialAlertMessages: readonly AlertMessageType[],
}

type State = {
  messages: readonly AlertMessageType[],
  alerts: ReactElement<typeof Alert>[],
}

class FlashMessages extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      messages: props.initialAlertMessages || [],
      alerts: [],
    }

    window.flashMessages = this
  }

  get alerts() {
    return this.state.messages.map((message) => (
      <Alert
        key={message.id}
        alert={message}
        onClose={() => this.removeMessage(message)}
      />
    ))
  }

  removeAlert = (component: ReactElement<typeof Alert>) => {
    const index = this.state.alerts.findIndex((comp) => comp === component)
    this.setState((prevState) => ({
      alerts: update(prevState.alerts, { $splice: [[index, 1]] }),
    }))
  }

  removeMessage = (alert: AlertMessageType) => {
    const index = this.state.messages.indexOf(alert)
    this.setState((prevState) => ({
      messages: update(prevState.messages, { $splice: [[index, 1]] }),
    }))
  }

  addMessage = (message: AlertMessageType) => {
    this.setState((prevState) => ({
      messages: update(prevState.messages, { $push: [message] }),
    }))
  }

  addAlert = (alert: ReactElement<typeof Alert>) => {
    this.setState((prevState) => ({
      alerts: update(prevState.alerts, { $push: [alert] }),
    }))
  }

  render = () => {
    const { alerts } = this

    if (alerts.length === 0 && this.state.alerts.length === 0) return null

    return (
      <div
        data-testid="alerts"
        className="container-lg clearfix my-2 px-3 px-md-6 px-lg-3"
      >
        {this.state.alerts}
        {alerts}
      </div>
    )
  }
}

export default FlashMessages
