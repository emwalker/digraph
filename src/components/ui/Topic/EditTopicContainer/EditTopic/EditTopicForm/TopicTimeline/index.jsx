// @flow
import React, { Fragment, useState, useCallback } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Relay } from 'components/types'
import upsertTopicTimelineMutation from 'mutations/upsertTopicTimelineMutation'
import type { TopicTimeline_topic as Topic } from './__generated__/TopicTimeline_topic.graphql'
import TopicTimelineForm from './TopicTimelineForm'
import styles from './styles.module.css'

type Props = {
  topic: Topic,
  relay: Relay,
}

const TopicTimeline = ({ relay, topic }: Props) => {
  const [mutationInFlight, setMutationInFlight] = useState(false)
  const checked = !!topic.timeline

  const onChange = useCallback(
    async (event: SyntheticInputEvent<HTMLInputElement>) => {
      setMutationInFlight(true)
      upsertTopicTimelineMutation(
        relay.environment,
        [],
        {
          topicId: topic.id,
          startsAt: (new Date()).toJSON(),
          prefixFormat: 'NONE',
        },
      )
      setMutationInFlight(false)
    },
    [setMutationInFlight],
  )

  return (
    <div className={styles.timelineForm}>
      <div className="form-checkbox mb-1">
        <label htmlFor="timeline-checkbox">
          <input
            checked={checked}
            disabled={mutationInFlight}
            id="timeline-checkbox"
            onChange={onChange}
            type="checkbox"
          />
          {' Has a timeline'}
        </label>
      </div>
      {checked && <TopicTimelineForm topic={topic} timeline={topic.timeline} />}
    </div>
  )
}

export default createFragmentContainer(TopicTimeline, {
  topic: graphql`
    fragment TopicTimeline_topic on Topic {
      id

      timeline {
        startsAt
      }

      ...TopicTimelineForm_topic
    }
  `
})
