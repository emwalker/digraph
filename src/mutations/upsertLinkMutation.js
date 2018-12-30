import { commitMutation, graphql } from 'react-relay'
import uuidv1 from 'uuid/v1'

import flashMessageUpdater from './util/flashMessageUpdater'
import updateTopicConnections from './util/updateTopicConnections'

let tmpId = 0

export default (environment, configs, input) => {
  const mutation = graphql`
    mutation upsertLinkMutation(
      $input: UpsertLinkInput!
    ) {
      upsertLink(input: $input) {
        alerts {
          text
          type
          id
        }

        linkEdge {
          node {
            ...Link_link
          }
        }
      }
    }
  `

  const optimisticUpdater = (store) => {
    tmpId += 1
    const id = `client:link:${tmpId}`
    const node = store.create(id, 'Link')
    node.setValue(id, 'id')
    node.setValue(input.title || 'Adding link to repo ...', 'title')
    node.setValue(input.url, 'url')
    updateTopicConnections(store, node, 'LinkEdge', input.addParentTopicIds || [], 'Topic_links')
  }

  return commitMutation(
    environment,
    {
      mutation,
      configs,
      optimisticUpdater,
      updater: flashMessageUpdater('upsertLink'),
      variables: {
        input: { clientMutationId: uuidv1(), ...input },
      },
    },
  )
}
