import type { Updater } from './types'

export default (rootField: string): Updater => (store) => {
  const payload = store.getRootField(rootField)
  if (!payload) return

  const alerts = payload.getLinkedRecords('alerts') || []

  for (let i = 0; i < alerts.length; i += 1) {
    const alert = alerts[i]
    window.flashMessages?.addMessage({
      text: alert.getValue('text'),
      type: alert.getValue('type'),
      id: alert.getValue('id'),
    })
  }
}
