// @flow
import React from 'react'

type Props = {
  onSave: () => void,
  onCancel: () => void,
}

export default ({ onSave, onCancel }: Props) => (
  <div>
    <button onClick={onSave} className="btn-primary">Save</button>
    {' '} or {' '}
    <button onClick={onCancel} className="btn-link">close</button>
  </div>
)
