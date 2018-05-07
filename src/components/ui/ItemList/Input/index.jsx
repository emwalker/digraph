// @flow
import React from 'react'

/* eslint jsx-a11y/label-has-for: 0 */

type Props = {
  className: string,
  id: string,
  label: string,
  value: string,
}

export default ({
  className, id, label, value,
}: Props) => (
  <div className={className}>
    <dl className="form-group">
      <dt>
        <label htmlFor={id}>{label}</label>
      </dt>
      <dd>
        <input
          id={id}
          type="text"
          className="form-control input-sm"
          defaultValue={value}
        />
      </dd>
    </dl>
  </div>
)
