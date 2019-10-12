// @flow
import React, { Component } from 'react'
import update from 'immutability-helper'

// https://medium.com/@veelenga/displaying-rails-flash-messages-with-react-5f82982f241c

import Alert from './Alert'

type AlertType = {
  +id: string,
}

type Props<A> = {
  initialAlerts: ?$ReadOnlyArray<A>,
}

type State<A> = {
  messages: $ReadOnlyArray<A>,
}

class FlashMessages<A: AlertType> extends Component<Props<A>, State<A>> {
  constructor(props: Props<A>) {
    super(props)
    this.state = {
      messages: props.initialAlerts || [],
    }
    window.flashMessages = this
  }

  get alerts(): Array<React$Node> {
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

  render = () => {
    const { alerts } = this

    if (alerts.length === 0) return null

    return (
      <div className="container-lg clearfix my-2 px-3 px-md-6 px-lg-3">
        { alerts }
      </div>
    )
  }
}

export default FlashMessages
