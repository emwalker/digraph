import React, { useState } from 'react'
import { createFragmentContainer, graphql, RelayProp } from 'react-relay'

import upsertTopicTimeRangeMutation, {
  Input as UpdateInput,
} from 'mutations/upsertTopicTimeRangeMutation'
import deleteTopicTimeRangeMutation, {
  Input as DeleteInput,
} from 'mutations/deleteTopicTimeRangeMutation'
import { TopicTimeRange_topic as TopicType } from '__generated__/TopicTimeRange_topic.graphql'
import TopicTimeRangeForm from './TopicTimeRangeForm'
import styles from './styles.module.css'

type Props = {
  topic: TopicType,
  relay: RelayProp,
}

const updateOrDelete = (
  relay: RelayProp,
  topic: TopicType,
  setMutationInFlight: (inFlight: boolean) => void,
) => async () => {
  setMutationInFlight(true)

  if (topic.timeRange) {
    const input: DeleteInput = { topicId: topic.id }
    await deleteTopicTimeRangeMutation(relay.environment, input)
  } else {
    const input: UpdateInput = {
      topicId: topic.id,
      startsAt: (new Date()).toISOString(),
      prefixFormat: 'START_YEAR_MONTH',
    }
    await upsertTopicTimeRangeMutation(relay.environment, input)
  }

  setMutationInFlight(false)
}

const TopicTimeRange = ({ relay, topic }: Props) => {
  const [mutationInFlight, setMutationInFlight] = useState(false)
  const checked = !!topic.timeRange

  const onChange = updateOrDelete(relay, topic, setMutationInFlight)

  return (
    <div className={styles.timelineForm}>
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
      {checked && <TopicTimeRangeForm topic={topic} />}
    </div>
  )
}

export default createFragmentContainer(TopicTimeRange, {
  topic: graphql`
    fragment TopicTimeRange_topic on Topic {
      id

      timeRange {
        startsAt
      }

      ...TopicTimeRangeForm_topic
    }
  `,
})
