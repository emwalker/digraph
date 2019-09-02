// @flow
import React, { useState, useCallback } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import moment from 'moment'
import { useDebouncedCallback } from 'use-debounce'

import type { Relay } from 'components/types'
import upsertTopicTimeRangeMutation from 'mutations/upsertTopicTimeRangeMutation'
import type { TopicTimelineForm_topic as Topic } from './__generated__/TopicTimeRangeForm_topic.graphql'
import styles from './styles.module.css'

// const initialDate = dateFormat(Date.now(), 'yyyy-mm-dd')

type TimeRange = $NonMaybeType<$PropertyType<Topic, 'timeRange'>>

type Props = {
  relay: Relay,
  timeRange: TimeRange,
  topic: Topic,
}

const TopicTimeRangeForm = ({ relay, timeRange, topic: { id: topicId } }: Props) => {
  const [mutationInFlight, setMutationInFlight] = useState(false)
  const [startsAt, setStartsAt] = useState(moment(timeRange.startsAt))
  const { prefixFormat } = timeRange

  const [updateStartsAt] = useDebouncedCallback(
    async (dt: Object) => {
      if (dt.isValid()) {
        setMutationInFlight(true)

        await upsertTopicTimeRangeMutation(
          relay.environment,
          [],
          {
            prefixFormat,
            startsAt: dt.toISOString(),
            topicId,
          },
        )
      } else {
        // eslint-disable-next-line no-console
        console.log('invalid date:', dt)
      }

      setMutationInFlight(false)
    },
    1000,
  )

  const updateFormat = useCallback(
    async (event: SyntheticInputEvent<HTMLInputElement>) => {
      setMutationInFlight(true)
      await upsertTopicTimeRangeMutation(
        relay.environment,
        [],
        {
          prefixFormat: event.target.value,
          startsAt,
          topicId,
        },
      )
      setMutationInFlight(false)
    },
    [setMutationInFlight, startsAt],
  )

  return (
    <div className={styles.formElements}>
      <dl className="form-group my-0">
        <dt><label htmlFor="time-range-prefix-format">Prefix</label></dt>
        <dd>
          <select
            className="form-select"
            disabled={mutationInFlight}
            id="time-range-prefix-select"
            onChange={updateFormat}
            value={prefixFormat}
          >
            <option value="NONE">None</option>
            <option value="START_YEAR">{startsAt.format('YYYY')}</option>
            <option value="START_YEAR_MONTH">{startsAt.format('YYYY-MM')}</option>
          </select>
        </dd>
      </dl>
      <dl className="form-group ml-3 my-0">
        <dt>
          <label htmlFor="time-range-starts-at">Start date</label>
        </dt>
        <dd>
          <input
            className={styles.startsAt}
            id="time-range-starts-at"
            onChange={(e) => {
              e.persist()
              const dt = moment(e.target.value)

              if (dt.isValid()) {
                setStartsAt(dt)
                updateStartsAt(dt)
              }
            }}
            required
            type="date"
            value={startsAt.isValid() ? startsAt.format('YYYY-MM-DD') : ''}
          />
        </dd>
      </dl>
    </div>
  )
}

export default createFragmentContainer(TopicTimeRangeForm, {
  timeRange: graphql`
    fragment TopicTimeRangeForm_timeRange on TimeRange {
      startsAt
      prefixFormat
    }
  `,
  topic: graphql`
    fragment TopicTimeRangeForm_topic on Topic {
      id
    }
  `,
})
