// @flow
import React, { useState } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Relay } from 'components/types'
import upsertTopicTimeRangeMutation, { type Input as UpdateInput } from 'mutations/upsertTopicTimeRangeMutation'
import deleteTopicTimeRangeMutation, { type Input as DeleteInput } from 'mutations/deleteTopicTimeRangeMutation'
import type { TopicTimeline_topic as Topic } from './__generated__/TopicTimeRange_topic.graphql'
import TopicTimeRangeForm from './TopicTimeRangeForm'
import styles from './styles.module.css'

type Props = {
  topic: Topic,
  relay: Relay,
}

const updateOrDelete = (relay, topic, setMutationInFlight) => async () => {
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
          {' Occurs in time'}
        </label>
      </div>
      {checked && (
        <TopicTimeRangeForm
          relay={relay}
          topic={topic}
          timeRange={topic.timeRange}
        />
      )}
    </div>
  )
}

export default createFragmentContainer(TopicTimeRange, {
  topic: graphql`
    fragment TopicTimeRange_topic on Topic {
      id

      timeRange {
        startsAt
        ...TopicTimeRangeForm_timeRange
      }

      ...TopicTimeRangeForm_topic
    }
  `,
})
