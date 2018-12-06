import { graphql } from 'react-relay'

import defaultMutation from './defaultMutation'

const updater = (store) => {
  const payload = store.getRootField('upsertTopic')
  const alerts = payload.getLinkedRecords('alerts')

  for (let i = 0; i < alerts.length; i += 1) {
    const alert = alerts[i]
    window.flashMessages.addMessage({
      text: alert.getValue('text'),
      type: alert.getValue('type'),
      id: alert.getValue('id'),
    })
  }
}

export default defaultMutation(graphql`
  mutation upsertTopicMutation(
    $input: UpsertTopicInput!
  ) {
    upsertTopic(input: $input) {
      alerts {
        text
        type
        id
      }

      topicEdge {
        node {
          ...Topic_topic
        }
      }
    }
  }
`, updater)
