import React, { useState, useCallback, FormEvent } from 'react'
import { createFragmentContainer, graphql, RelayProp } from 'react-relay'
import moment, { Moment } from 'moment/moment'
import { useDebouncedCallback } from 'use-debounce'

import upsertTopicTimeRangeMutation, { Input } from 'mutations/upsertTopicTimeRangeMutation'
import {
  TopicTimeRangeForm_topic as TopicType,
} from '__generated__/TopicTimeRangeForm_topic.graphql'
import styles from './styles.module.css'

type PrefixFormat = NonNullable<TopicType['timeRange']>['prefixFormat']

type Props = {
  relay: RelayProp,
  topic: TopicType,
}

const TopicTimeRangeForm = ({ relay, topic: { id: topicId, timeRange } }: Props) => {
  const [mutationInFlight, setMutationInFlight] = useState(false)
  const [startsAt, setStartsAt] = useState(moment(timeRange?.startsAt as string))
  const prefixFormat = timeRange?.prefixFormat

  const [updateStartsAt] = useDebouncedCallback(
    async (dt: Moment) => {
      if (dt.isValid() && prefixFormat) {
        setMutationInFlight(true)
        const input: Input = {
          prefixFormat,
          startsAt: dt.toISOString(),
          topicId,
        }
        await upsertTopicTimeRangeMutation(relay.environment, input)
      } else {
        // eslint-disable-next-line no-console
        console.log('invalid date:', dt)
      }

      setMutationInFlight(false)
    },
    1000,
  )

  const updateFormat = useCallback(
    async (event: FormEvent<HTMLSelectElement>) => {
      setMutationInFlight(true)
      const newPrefix = event.currentTarget.value as PrefixFormat
      const input: Input = {
        prefixFormat: newPrefix,
        startsAt,
        topicId,
      }
      await upsertTopicTimeRangeMutation(relay.environment, input)
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
  topic: graphql`
    fragment TopicTimeRangeForm_topic on Topic {
      id

      timeRange {
        startsAt
        prefixFormat
      }
    }
  `,
})
