import React, { useState, useCallback, FormEvent } from 'react'
import { graphql, useFragment, useMutation } from 'react-relay'
import moment, { Moment } from 'moment/moment'
import { useDebouncedCallback } from 'use-debounce'

import timerangeQuery from 'mutations/upsertTopicTimerangeMutation'
import {
  RepoTopicTimerangeForm_viewer$key,
} from '__generated__/RepoTopicTimerangeForm_viewer.graphql'
import {
  RepoTopicTimerangeForm_repoTopic$key,
  RepoTopicTimerangeForm_repoTopic$data as RepoTopic,
} from '__generated__/RepoTopicTimerangeForm_repoTopic.graphql'

type PrefixFormat = NonNullable<NonNullable<RepoTopic['details']>['timerange']>['prefixFormat']

type Props = {
  disabled: boolean,
  repoTopic: RepoTopicTimerangeForm_repoTopic$key,
  viewer: RepoTopicTimerangeForm_viewer$key,
}

const repoTopicFragment = graphql`
  fragment RepoTopicTimerangeForm_repoTopic on RepoTopic {
    topicId

    details {
      timerange {
        startsAt
        prefixFormat
      }
    }
  }
`

const viewerFragment = graphql`
  fragment RepoTopicTimerangeForm_viewer on User {
    selectedRepoId
  }
`

export default function RepoTopicTimerangeForm(props: Props) {
  const repoTopic = useFragment(repoTopicFragment, props.repoTopic)
  const viewer = useFragment(viewerFragment, props.viewer)

  const [upsertTopicTimerangeMutation, upsertTimerangeInFlight] = useMutation(timerangeQuery)

  const timerange = repoTopic.details?.timerange
  const [startsAt, setStartsAt] = useState(moment(timerange?.startsAt as string))
  const prefixFormat = timerange?.prefixFormat
  const repoId = viewer.selectedRepoId

  const updateStartsAt = useDebouncedCallback(
    (dt: Moment) => {
      if (dt.isValid() && prefixFormat) {
        if (!repoId) {
          console.log('no repo selected')
          return
        }

        upsertTopicTimerangeMutation({
          variables: {
            input: {
              prefixFormat,
              repoId,
              startsAt: dt.toISOString(),
              topicId: repoTopic.topicId,
            },
          },
        })
      } else {
        // eslint-disable-next-line no-console
        console.log('invalid date:', dt)
      }
    },
    1000,
  )

  const updateFormat = useCallback(
    (event: FormEvent<HTMLSelectElement>) => {
      if (!repoId) {
        console.log('no repo selected')
        return
      }

      const newPrefix = event.currentTarget.value as PrefixFormat

      upsertTopicTimerangeMutation({
        variables: {
          input: {
            prefixFormat: newPrefix,
            repoId,
            startsAt,
            topicId: repoTopic.topicId,
          },
        },
      })
    },
    [upsertTopicTimerangeMutation, startsAt],
  )

  const disabled = props.disabled || upsertTimerangeInFlight

  return (
    <div className="topicTimeRangeFormFormElements" data-testid="timerange-form">
      <dl className="form-group my-0">
        <dt><label htmlFor="time-range-prefix-format">Prefix</label></dt>
        <dd>
          <select
            className="form-select"
            disabled={disabled}
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
            disabled={disabled}
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
