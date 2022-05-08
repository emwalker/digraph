import React, { FormEvent } from 'react'

type Props = {
  className: string,
  id: string,
  label: string,
  onChange: (event: FormEvent<HTMLInputElement>) => void,
  value: string | undefined,
}

export default ({ className, id, label, onChange, value }: Props) => (
  <div className={className}>
    <dl className="form-group">
      <dt>
        <label htmlFor={id}>{label}</label>
      </dt>
      <dd>
        <input
          className="form-control"
          defaultValue={value || ''}
          id={id}
          onChange={onChange}
          type="text"
        />
      </dd>
    </dl>
  </div>
)
