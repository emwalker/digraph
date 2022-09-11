import React, { useState, useCallback } from 'react'
import { Environment, graphql, useFragment, useRelayEnvironment } from 'react-relay'

import upsertTopicTimerangeMutation, {
  Input as UpdateInput,
} from 'mutations/upsertTopicTimerangeMutation'
import removeTopicTimerangeMutation, {
  Input as DeleteInput,
} from 'mutations/removeTopicTimerangeMutation'
import {
  RepoTopicTimerange_repoTopic$key as RepoTopicKeyType,
  RepoTopicTimerange_repoTopic$data as RepoTopicType,
} from '__generated__/RepoTopicTimerange_repoTopic.graphql'
import RepoTopicTimerangeForm from './RepoTopicTimerangeForm'

type Props = {
  repoTopic: RepoTopicKeyType,
  viewer: any,
}

function updateOrDelete(
  environment: Environment,
  repoId: string | undefined,
  repoTopic: RepoTopicType,
  setMutationInFlight: (inFlight: boolean) => void,
) {
  if (!repoId) {
    console.log('no repo selected')
    return
  }

  setMutationInFlight(true)

  if (repoTopic.timerange) {
    const input: DeleteInput = { repoId, topicId: repoTopic.topicId }
    removeTopicTimerangeMutation(environment, input)
  } else {
    const input: UpdateInput = {
      prefixFormat: 'START_YEAR_MONTH',
      repoId,
      startsAt: (new Date()).toISOString(),
      topicId: repoTopic.topicId,
    }
    upsertTopicTimerangeMutation(environment, input)
  }

  setMutationInFlight(false)
}

export default function RepoTopicTimeRange(props: Props) {
  const [mutationInFlight, setMutationInFlight] = useState(false)

  const repoTopic = useFragment(
    graphql`
      fragment RepoTopicTimerange_repoTopic on RepoTopic {
        topicId

        timerange {
          startsAt
        }

        ...RepoTopicTimerangeForm_repoTopic
      }
    `,
    props.repoTopic,
  )

  const viewer = useFragment(
    graphql`
      fragment RepoTopicTimerange_viewer on User {
        selectedRepository {
          id
        }
        ...RepoTopicTimerangeForm_viewer
      }
    `,
    props.viewer,
  )

  const environment = useRelayEnvironment()
  const checked = !!repoTopic.timerange
  const repoId = viewer.selectedRepository?.id
  const onChange = useCallback(() => {
    updateOrDelete(environment, repoId, repoTopic, setMutationInFlight)
  }, [updateOrDelete, environment, repoId, repoTopic, setMutationInFlight])

  return (
    <div>
      <div className="form-checkbox mb-1">
        <label htmlFor="time-range-checkbox">
          <input
            checked={checked}
            disabled={mutationInFlight}
            id="time-range-checkbox"
            onChange={onChange}
            type="checkbox"
          />
          Occurs in time
        </label>
      </div>

      {checked && <RepoTopicTimerangeForm viewer={props.viewer} repoTopic={repoTopic} />}
    </div>
  )
}
