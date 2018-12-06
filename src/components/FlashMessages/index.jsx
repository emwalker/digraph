// @flow
import React, { Component } from 'react'
import update from 'immutability-helper'

// https://medium.com/@veelenga/displaying-rails-flash-messages-with-react-5f82982f241c

import Alert from './Alert'

type Props = {
  message: ?string,
}

type State = {
  messages: string[],
}

class FlashMessages extends Component<Props, State> {
  static defaultProps = {
    message: null,
  }

  constructor(props: Props) {
    super(props)
    this.state = {
      messages: props.message ? [props.message] : [],
    }
    window.flashMessages = this
  }

  get alerts() {
    return this.state.messages.map(message => (
      <Alert
        key={message.id}
        message={message}
        onClose={() => this.removeMessage(message)}
      />
    ))
  }

  removeMessage = (message) => {
    const index = this.state.messages.indexOf(message)
    const messages = update(this.state.messages, { $splice: [[index, 1]] })
    this.setState({ messages })
  }

  addMessage = (message: string) => {
    const messages = update(this.state.messages, { $push: [message] })
    this.setState({ messages })
  }

  render = () => (
    <div>
      { this.alerts }
    </div>
  )
}

export default FlashMessages
