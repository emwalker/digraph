import React, { useState } from 'react'
import { createFragmentContainer, graphql, RelayProp } from 'react-relay'

import { wikiRepoId } from 'components/constants'
import upsertTopicTimerangeMutation, {
  Input as UpdateInput,
} from 'mutations/upsertTopicTimerangeMutation'
import removeTopicTimerangeMutation, {
  Input as DeleteInput,
} from 'mutations/removeTopicTimerangeMutation'
import {
  TopicTimerange_repoTopic as RepoTopicType,
} from '__generated__/TopicTimerange_repoTopic.graphql'
import TopicTimerangeForm from './TopicTimerangeForm'

type Props = {
  topicDetail: RepoTopicType,
  relay: RelayProp,
}

const updateOrDelete = (
  relay: RelayProp,
  topicDetail: RepoTopicType,
  setMutationInFlight: (inFlight: boolean) => void,
) => async () => {
  setMutationInFlight(true)

  if (topicDetail.timerange) {
    const input: DeleteInput = { repoId: wikiRepoId, topicId: topicDetail.topicId }
    await removeTopicTimerangeMutation(relay.environment, input)
  } else {
    const input: UpdateInput = {
      prefixFormat: 'START_YEAR_MONTH',
      // FIXME
      repoId: wikiRepoId,
      startsAt: (new Date()).toISOString(),
      topicId: topicDetail.topicId,
    }
    await upsertTopicTimerangeMutation(relay.environment, input)
  }

  setMutationInFlight(false)
}

const TopicTimeRange = ({ relay, topicDetail }: Props) => {
  const [mutationInFlight, setMutationInFlight] = useState(false)
  const checked = !!topicDetail.timerange

  const onChange = updateOrDelete(relay, topicDetail, setMutationInFlight)

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
      {checked && <TopicTimerangeForm topicDetail={topicDetail} />}
    </div>
  )
}

export default createFragmentContainer(TopicTimeRange, {
  repoTopic: graphql`
    fragment TopicTimerange_repoTopic on RepoTopic {
      topicId

      timerange {
        startsAt
      }

      ...TopicTimerangeForm_repoTopic
    }
  `,
})
