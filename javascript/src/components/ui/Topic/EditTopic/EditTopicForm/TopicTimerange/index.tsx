import React, { useState } from 'react'
import { graphql, useFragment, useRelayEnvironment } from 'react-relay'

import { wikiRepoId } from 'components/constants'
import upsertTopicTimerangeMutation, {
  Input as UpdateInput,
} from 'mutations/upsertTopicTimerangeMutation'
import removeTopicTimerangeMutation, {
  Input as DeleteInput,
} from 'mutations/removeTopicTimerangeMutation'
import {
  TopicTimerange_repoTopic$key as RepoTopicKeyType,
  TopicTimerange_repoTopic$data as RepoTopicType,
} from '__generated__/TopicTimerange_repoTopic.graphql'
import TopicTimerangeForm from './TopicTimerangeForm'

type Props = {
  repoTopic: RepoTopicKeyType,
}

const updateOrDelete = (
  repoTopic: RepoTopicType,
  setMutationInFlight: (inFlight: boolean) => void,
) => async () => {
  const environment = useRelayEnvironment()
  setMutationInFlight(true)

  if (repoTopic.timerange) {
    const input: DeleteInput = { repoId: wikiRepoId, topicId: repoTopic.topicId }
    await removeTopicTimerangeMutation(environment, input)
  } else {
    const input: UpdateInput = {
      prefixFormat: 'START_YEAR_MONTH',
      // FIXME
      repoId: wikiRepoId,
      startsAt: (new Date()).toISOString(),
      topicId: repoTopic.topicId,
    }
    await upsertTopicTimerangeMutation(environment, input)
  }

  setMutationInFlight(false)
}

const TopicTimeRange = ({ repoTopic }: Props) => {
  const [mutationInFlight, setMutationInFlight] = useState(false)

  const data = useFragment(graphql`
    fragment TopicTimerange_repoTopic on RepoTopic {
      topicId

      timerange {
        startsAt
      }

      ...TopicTimerangeForm_repoTopic
    }
  `, repoTopic)

  const checked = !!data.timerange
  const onChange = updateOrDelete(data, setMutationInFlight)

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
      {
        checked &&
        // @ts-expect-error
        <TopicTimerangeForm repoTopic={data} />
      }
    </div>
  )
}

export default TopicTimeRange
