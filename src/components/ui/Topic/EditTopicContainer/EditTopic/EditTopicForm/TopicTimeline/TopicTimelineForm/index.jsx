// @flow
import React, { useCallback, useState } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'
import dateFormat from 'dateformat'

import type { TopicTimelineForm_topic as Topic } from './__generated__/TopicTimelineForm_topic.graphql'
import styles from './styles.module.css'

const initialDate = dateFormat(Date.now(), 'yyyy-mm-dd')

type Timeline = $NonMaybeType<$PropertyType<Topic, 'timeline'>>

type Props = {
  topic: Topic,
  timeline: Timeline,
}

const TopicTimelineForm = ({ topic, timeline }: Props) => {
  const [startsAt, setStartsAt] = useState(Date.parse(timeline.startsAt))
  const [format, setFormat] = useState(timeline.prefixFormat)

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
        <dt><label htmlFor="timeline-prefix-format">Prefix</label></dt>
        <dd>
          <select
            className="form-select"
            id="timeline-prefix-select"
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
          <label htmlFor="timeline-starts-at">Start date</label>
        </dt>
        <dd>
          <input
            className={styles.startsAt}
            id="timeline-starts-at"
            onChange={updateStartsAt}
            type="date"
            value={startsAt}
          />
        </dd>
      </dl>
    </div>
  )
}

export default createFragmentContainer(TopicTimelineForm, {
  topic: graphql`
    fragment TopicTimelineForm_topic on Topic {
      id

      timeline {
        startsAt
        prefixFormat
      }
    }
  `
})
