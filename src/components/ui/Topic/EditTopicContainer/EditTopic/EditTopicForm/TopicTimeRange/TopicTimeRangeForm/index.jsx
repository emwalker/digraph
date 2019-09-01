// @flow
import React, { useCallback, useState } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import dateFormat from 'dateformat'

import type { TopicTimelineForm_topic as Topic } from './__generated__/TopicTimeRangeForm_topic.graphql'
import styles from './styles.module.css'

// const initialDate = dateFormat(Date.now(), 'yyyy-mm-dd')

type TimeRange = $NonMaybeType<$PropertyType<Topic, 'timeRange'>>

type Props = {
  // topic: Topic,
  timeRange: TimeRange,
}

const TopicTimeRangeForm = ({ timeRange }: Props) => {
  const [startsAt, setStartsAt] = useState(Date.parse(timeRange.startsAt))
  const [format, setFormat] = useState(timeRange.prefixFormat)

  const updateStartsAt = useCallback(
    (event: SyntheticInputEvent<HTMLInputElement>) => setStartsAt(event.target.value),
    [setStartsAt],
  )

  const updateFormat = useCallback(
    (event: SyntheticInputEvent<HTMLSelectElement>) => setFormat(event.target.value),
    [setFormat],
  )

  return (
    <div className={styles.formElements}>
      <dl className="form-group my-0">
        <dt><label htmlFor="time-range-prefix-format">Prefix</label></dt>
        <dd>
          <select
            className="form-select"
            id="time-range-prefix-select"
            onChange={updateFormat}
            value={format}
          >
            <option value="NONE">Not shown</option>
            <option value="START_YEAR">{dateFormat(startsAt, 'yyyy')}</option>
            <option value="START_YEAR_MONTH">{dateFormat(startsAt, 'yyyy-mm')}</option>
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
            onChange={updateStartsAt}
            type="date"
            value={startsAt}
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
