import React, { MouseEventHandler, type ReactNode } from 'react'
import { GoX } from 'react-icons/go'

import { AlertMessageType } from 'components/types'

// https://medium.com/@veelenga/displaying-rails-flash-messages-with-react-5f82982f241c

export type OnCloseType = MouseEventHandler<HTMLButtonElement>

type Props = {
  children?: ReactNode,
  alert: AlertMessageType,
  onClose?: OnCloseType,
}

const alertCasses = {
  ERROR: 'flash-error',
  WARN: 'flash-warn',
  SUCCESS: 'flash-success',
  '%future added value': 'flash-error',
}

export default function Alert({ children, alert, onClose }: Props) {
  const alertClass = alertCasses[alert.type] || 'flash-success'

  const defaultOnClose = () => {
    const removeAlert = window.flashMessages?.removeAlert
    if (!removeAlert) return
    removeAlert(alert.id)
  }

  return (
    <div className={`flash fade in mt-3 mb-3 ${alertClass}`}>
      <button
        className="flash-close"
        onClick={onClose || defaultOnClose}
        type="button"
      >
        <GoX />
      </button>
      { alert.text }
      { children }
    </div>
  )
}
