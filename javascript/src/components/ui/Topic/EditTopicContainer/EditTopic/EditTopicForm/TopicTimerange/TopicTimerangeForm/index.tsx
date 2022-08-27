import React, { useState, useCallback, FormEvent } from 'react'
import { createFragmentContainer, graphql, RelayProp } from 'react-relay'
import moment, { Moment } from 'moment/moment'
import { useDebouncedCallback } from 'use-debounce'

import upsertTopicTimerangeMutation, { Input } from 'mutations/upsertTopicTimerangeMutation'
import {
  TopicTimerangeForm_topic as TopicType,
} from '__generated__/TopicTimerangeForm_topic.graphql'

type PrefixFormat = NonNullable<TopicType['timerange']>['prefixFormat']

type Props = {
  relay: RelayProp,
  topic: TopicType,
}

const TopicTimerangeForm = ({ relay, topic: { id: topicId, timerange } }: Props) => {
  const [mutationInFlight, setMutationInFlight] = useState(false)
  const [startsAt, setStartsAt] = useState(moment(timerange?.startsAt as string))
  const prefixFormat = timerange?.prefixFormat

  const updateStartsAt = useDebouncedCallback(
    (dt: Moment) => {
      if (dt.isValid() && prefixFormat) {
        setMutationInFlight(true)

        // FIXME: get repo from selected repo
        const repoId = '/wiki/'

        const input: Input = {
          prefixFormat,
          repoId,
          startsAt: dt.toISOString(),
          topicId,
        }
        upsertTopicTimerangeMutation(relay.environment, input)
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

      // FIXME: get repo from selected repo
      const repoId = '/wiki/'

      const input: Input = {
        prefixFormat: newPrefix,
        repoId,
        startsAt,
        topicId,
      }
      await upsertTopicTimerangeMutation(relay.environment, input)
      setMutationInFlight(false)
    },
    [setMutationInFlight, startsAt],
  )

  return (
    <div className="topicTimeRangeFormFormElements">
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
            className="topicTimeRangeFormStartsAt"
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

export default createFragmentContainer(TopicTimerangeForm, {
  topic: graphql`
    fragment TopicTimerangeForm_topic on Topic {
      id

      timerange {
        startsAt
        prefixFormat
      }
    }
  `,
})
