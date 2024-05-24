import React, { FormEvent } from 'react'

type Props = {
  className: string,
  disabled?: boolean,
  id: string,
  label: string,
  onChange?: (event: FormEvent<HTMLInputElement>) => void,
  placeholder?: string,
  value: string | undefined,
}

export default ({ className, disabled, id, label, onChange, placeholder, value }: Props) => (
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
          disabled={disabled}
          onChange={onChange}
          placeholder={placeholder}
          type="text"
        />
      </dd>
    </dl>
  </div>
)
