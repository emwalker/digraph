// @flow
import React, { Component } from 'react'

import Input from '../../Input'

type Props = {
  id: string,
  display: string,
  isOpen: boolean,
  toggleFn: Function,
  url: string,
}

type State = {
  title: string,
  url: string,
}

class EditLink extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      title: props.display,
      url: props.url,
    }
  }

  onSave = () => {
    this.props.toggleFn()
  }

  render() {
    if (!this.props.isOpen)
      return null

    return (
      <div>
        <div className="d-flex col-12">
          <Input
            className="col-6"
            id={`edit-link-title-${this.props.id}`}
            label="Page title"
            value={this.state.title}
          />
          <Input
            className="col-6"
            id={`edit-link-url-${this.props.id}`}
            label="Url"
            value={this.state.url}
          />
        </div>
        <div>
          <button onClick={this.onSave} className="btn-primary">Save</button>
          {' '} or {' '}
          <button onClick={this.props.toggleFn} className="btn-link">cancel</button>
        </div>
      </div>
    )
  }
}

export default EditLink
