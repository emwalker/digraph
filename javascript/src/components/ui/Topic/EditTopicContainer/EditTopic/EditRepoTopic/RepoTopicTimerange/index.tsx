import React, { useCallback } from 'react'
import { graphql, useFragment, useMutation } from 'react-relay'

import upsertQuery from 'mutations/upsertTopicTimerangeMutation'
import removeQuery from 'mutations/removeTopicTimerangeMutation'
import {
  RepoTopicTimerange_repoTopic$key as RepoTopicKeyType,
} from '__generated__/RepoTopicTimerange_repoTopic.graphql'
import RepoTopicTimerangeForm from './RepoTopicTimerangeForm'
import { upsertTopicTimerangeMutation } from '__generated__/upsertTopicTimerangeMutation.graphql'
import { removeTopicTimerangeMutation } from '__generated__/removeTopicTimerangeMutation.graphql'

type Props = {
  repoTopic: RepoTopicKeyType,
  viewer: any,
}

const repoTopicFragment = graphql`
  fragment RepoTopicTimerange_repoTopic on RepoTopic {
    topicId

    timerange {
      startsAt
    }

    ...RepoTopicTimerangeForm_repoTopic
  }
`

const viewerFragment = graphql`
  fragment RepoTopicTimerange_viewer on User {
    selectedRepository {
      id
    }
    ...RepoTopicTimerangeForm_viewer
  }
`

export default function RepoTopicTimeRange(props: Props) {
  const repoTopic = useFragment(repoTopicFragment, props.repoTopic)
  const viewer = useFragment(viewerFragment, props.viewer)
  const [upsertTopicTimerange, upsertTimerangeInFlight] =
    useMutation<upsertTopicTimerangeMutation>(upsertQuery)
  const [removeTopicTimerange, removeTimerangeInFlight] =
    useMutation<removeTopicTimerangeMutation>(removeQuery)

  const checked = !!repoTopic.timerange
  const repoId = viewer.selectedRepository?.id

  const onChange = useCallback(() => {
    if (!repoId) {
      console.log('no repo selected')
      return
    }

    if (repoTopic.timerange) {
      removeTopicTimerange({
        variables: {
          input: { repoId, topicId: repoTopic.topicId },
        },
      })
    } else {
      upsertTopicTimerange({
        variables: {
          input: {
            prefixFormat: 'START_YEAR',
            repoId,
            startsAt: (new Date()).toISOString(),
            topicId: repoTopic.topicId,
          },
        },
      })
    }

  }, [upsertTopicTimerange, repoId, repoTopic])

  return (
    <div>
      <div className="form-checkbox mb-1">
        <label htmlFor="time-range-checkbox">
          <input
            checked={checked}
            disabled={upsertTimerangeInFlight || removeTimerangeInFlight}
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
