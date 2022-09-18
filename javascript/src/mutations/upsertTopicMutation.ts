import { Dispatch, SetStateAction, useCallback, KeyboardEvent  } from 'react'
import { DeclarativeMutationConfig, graphql, useMutation } from 'react-relay'
import { upsertTopicMutation } from '__generated__/upsertTopicMutation.graphql'

function relayConfigs(parentID: string) {
  return [{
    type: 'RANGE_ADD',
    parentID,
    connectionInfo: [{
      key: 'Topic_children',
      rangeBehavior: 'prepend',
    }],
    edgeName: 'topicEdge',
  }]
}

const query = graphql`
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
`

export function makeUpsertTopic({ selectedRepoId, name, setName, topicId }: {
  name: string,
  selectedRepoId: string | null,
  setName: Dispatch<SetStateAction<string>>,
  topicId: string,
}) {
  const upsertTopic = useMutation<upsertTopicMutation>(query)[0]

  return useCallback((event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key !== 'Enter') return

    if (!selectedRepoId) {
      // eslint-disable-next-line no-console
      console.log('repo not selected')
      return
    }
    
    upsertTopic({
      variables: {
        input: {
          name,
          repoId: selectedRepoId,
          parentTopicId: topicId,
        },
      },
      configs: relayConfigs(topicId) as DeclarativeMutationConfig[],
    })
  
    setName('')
  }, [upsertTopic, selectedRepoId, topicId, name, setName])
}
