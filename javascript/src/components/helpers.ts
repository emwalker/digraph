import { AlertMessageType } from './types'

function topicPath(id: string): string {
  return `/topics/${id}`
}

function backgroundColor(hexColor: string) {
  return hexColor === '' ? '' : `${hexColor}33`
}

function borderColor(hexColor: string) {
  return hexColor === '' ? '' : `${hexColor}ff`
}

type GetAlertsType<R> = (response: R) => readonly AlertMessageType[]

function showAlerts<R>(getAlerts: GetAlertsType<R>): (response: R) => void {
  return (response: R) => {
    const alerts = getAlerts(response)
    const addMessage = window.flashMessages?.addMessage
    if (addMessage == null || alerts.length === 0) return
    alerts.forEach(addMessage)
  }
}

export { topicPath, backgroundColor, borderColor, showAlerts }
