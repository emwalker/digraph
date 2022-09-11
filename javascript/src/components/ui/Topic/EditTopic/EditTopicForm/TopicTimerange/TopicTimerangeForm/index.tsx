import React, { useState, useCallback, FormEvent } from 'react'
import { graphql, useFragment, useRelayEnvironment } from 'react-relay'
import moment, { Moment } from 'moment/moment'
import { useDebouncedCallback } from 'use-debounce'

import upsertTopicTimerangeMutation, { Input } from 'mutations/upsertTopicTimerangeMutation'
import {
  TopicTimerangeForm_repoTopic$key,
  TopicTimerangeForm_repoTopic$data as RepoTopic,
} from '__generated__/TopicTimerangeForm_repoTopic.graphql'
import { wikiRepoId } from 'components/constants'

type PrefixFormat = NonNullable<RepoTopic['timerange']>['prefixFormat']

type Props = {
  repoTopic: TopicTimerangeForm_repoTopic$key,
}

export default function TopicTimerangeForm(props: Props) {
  const repoTopic = useFragment(
    graphql`
      fragment TopicTimerangeForm_repoTopic on RepoTopic {
        topicId

        timerange {
          startsAt
          prefixFormat
        }
      }
    `,
    props.repoTopic,
  )

  const [mutationInFlight, setMutationInFlight] = useState(false)
  const [startsAt, setStartsAt] = useState(moment(repoTopic.timerange?.startsAt as string))
  const prefixFormat = repoTopic.timerange?.prefixFormat
  const environment = useRelayEnvironment()

  const updateStartsAt = useDebouncedCallback(
    (dt: Moment) => {
      if (dt.isValid() && prefixFormat) {
        setMutationInFlight(true)

        const input: Input = {
          prefixFormat,
          // FIXME
          repoId: wikiRepoId,
          startsAt: dt.toISOString(),
          topicId: repoTopic.topicId,
        }
        upsertTopicTimerangeMutation(environment, input)
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
        // FIXME
        repoId: wikiRepoId,
        startsAt,
        topicId: repoTopic.topicId,
      }
      await upsertTopicTimerangeMutation(environment, input)
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
