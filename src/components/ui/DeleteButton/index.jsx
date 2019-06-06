// @flow
import React, { Component } from 'react'
import classNames from 'classnames'

type Props = {
  className?: string,
  onDelete: () => void,
}

class DeleteButton extends Component<Props> {
  static defaultProps = {
    className: '',
  }

  get className(): string {
    return classNames('btn btn-sm btn-danger', this.props.className)
  }

  confirmAndDelete = () => {
    // eslint-disable-next-line no-alert
    if (!window.confirm('Are you sure you want to delete this item?')) return
    this.props.onDelete()
  }

  render = () => (
    <button onClick={this.confirmAndDelete} className={this.className} type="submit">
      Delete
    </button>
  )
}

export default DeleteButton
