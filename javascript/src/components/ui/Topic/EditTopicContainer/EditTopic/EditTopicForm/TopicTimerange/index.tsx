import React, { useState } from 'react'
import { createFragmentContainer, graphql, RelayProp } from 'react-relay'

import upsertTopicTimerangeMutation, {
  Input as UpdateInput,
} from 'mutations/upsertTopicTimerangeMutation'
import removeTopicTimerangeMutation, {
  Input as DeleteInput,
} from 'mutations/removeTopicTimerangeMutation'
import { TopicTimerange_topic as TopicType } from '__generated__/TopicTimerange_topic.graphql'
import TopicTimerangeForm from './TopicTimerangeForm'

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

  if (topic.timerange) {
    const input: DeleteInput = { topicPath: topic.path }
    await removeTopicTimerangeMutation(relay.environment, input)
  } else {
    const input: UpdateInput = {
      topicPath: topic.path,
      startsAt: (new Date()).toISOString(),
      prefixFormat: 'START_YEAR_MONTH',
    }
    await upsertTopicTimerangeMutation(relay.environment, input)
  }

  setMutationInFlight(false)
}

const TopicTimeRange = ({ relay, topic }: Props) => {
  const [mutationInFlight, setMutationInFlight] = useState(false)
  const checked = !!topic.timerange

  const onChange = updateOrDelete(relay, topic, setMutationInFlight)

  return (
    <div>
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
      {checked && <TopicTimerangeForm topic={topic} />}
    </div>
  )
}

export default createFragmentContainer(TopicTimeRange, {
  topic: graphql`
    fragment TopicTimerange_topic on Topic {
      id
      path

      timerange {
        startsAt
      }

      ...TopicTimerangeForm_topic
    }
  `,
})
