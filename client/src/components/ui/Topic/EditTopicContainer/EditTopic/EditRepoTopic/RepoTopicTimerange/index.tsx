import React, { useCallback } from 'react'
import { Disposable, graphql, useFragment, useMutation, UseMutationConfig } from 'react-relay'

import upsertQuery from 'mutations/upsertTopicTimerangeMutation'
import removeQuery from 'mutations/removeTopicTimerangeMutation'
import {
  RepoTopicTimerange_repoTopic$key,
  RepoTopicTimerange_repoTopic$data as RepoTopicKeyType,
} from '__generated__/RepoTopicTimerange_repoTopic.graphql'
import RepoTopicTimerangeForm from './RepoTopicTimerangeForm'
import { upsertTopicTimerangeMutation } from '__generated__/upsertTopicTimerangeMutation.graphql'
import { removeTopicTimerangeMutation } from '__generated__/removeTopicTimerangeMutation.graphql'

type Props = {
  repoTopic: RepoTopicTimerange_repoTopic$key,
  viewer: any,
}

function makeOnChange({ upsertTopicTimerange, removeTopicTimerange, repoId, repoTopic }: {
  upsertTopicTimerange: (config: UseMutationConfig<upsertTopicTimerangeMutation>) => Disposable,
  removeTopicTimerange: (config: UseMutationConfig<removeTopicTimerangeMutation>) => Disposable,
  repoId: string,
  repoTopic: RepoTopicKeyType,
}) {
  return useCallback(() => {
    if (!repoId) {
      console.log('no repo selected')
      return
    }

    if (repoTopic.details?.timerange) {
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
}

const repoTopicFragment = graphql`
  fragment RepoTopicTimerange_repoTopic on RepoTopic {
    id
    topicId

    details {
      timerange {
        startsAt
      }
    }

    ...RepoTopicTimerangeForm_repoTopic
  }
`

const viewerFragment = graphql`
  fragment RepoTopicTimerange_viewer on User {
    selectedRepoId
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

  const checked = !!repoTopic.details?.timerange
  const repoId = viewer.selectedRepoId
  const onChange = makeOnChange({ upsertTopicTimerange, removeTopicTimerange, repoId, repoTopic })
  const disabled = upsertTimerangeInFlight || removeTimerangeInFlight

  return (
    <div>
      <div className="form-checkbox mb-1">
        <label htmlFor="timerange-checkbox">
          <input
            checked={checked}
            data-testid="timerange-checkbox"
            disabled={disabled}
            id="timerange-checkbox"
            onChange={onChange}
            type="checkbox"
          />
          Occurs in time
        </label>
      </div>

      {checked && (
        <RepoTopicTimerangeForm
          viewer={viewer}
          repoTopic={repoTopic}
          disabled={disabled}
        />
      )}
    </div>
  )
}
