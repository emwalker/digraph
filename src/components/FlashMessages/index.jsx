// @flow
import React, { Component } from 'react'
import update from 'immutability-helper'

// https://medium.com/@veelenga/displaying-rails-flash-messages-with-react-5f82982f241c

import type { AlertType } from 'components/types'
import Alert from './Alert'

type Props = {
  initialAlerts: ?AlertType[],
}

type State = {
  messages: AlertType[],
}

class FlashMessages extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      messages: props.initialAlerts || [],
    }
    window.flashMessages = this
  }

  get alerts(): Iterable<React$Node> {
    return this.state.messages.map(message => (
      <Alert
        key={message.id}
        message={message}
        onClose={() => this.removeMessage(message)}
      />
    ))
  }

  removeMessage = (message: AlertType) => {
    const index = this.state.messages.indexOf(message)
    this.setState(prevState => ({
      messages: update(prevState.messages, { $splice: [[index, 1]] }),
    }))
  }

  addMessage = (message: AlertType) => {
    this.setState(prevState => ({
      messages: update(prevState.messages, { $push: [message] }),
    }))
  }

  render = () => (
    <div>
      { this.alerts }
    </div>
  )
}

export default FlashMessages
