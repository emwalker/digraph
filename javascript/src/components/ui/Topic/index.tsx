import React, { useState, Suspense } from 'react'
import { graphql, useFragment } from 'react-relay'

import { topicPath } from 'components/helpers'
import { NodeTypeOf, liftNodes } from 'components/types'
import { Topic_topic$data, Topic_topic$key } from '__generated__/Topic_topic.graphql'
import Item from '../Item'
import EditTopic from './EditTopic'

type ParentTopicType = NodeTypeOf<Topic_topic$data['displayParentTopics']>

type Props = {
  topic: Topic_topic$key,
}

export default function Topic(props: Props) {
  const [formIsOpen, setFormIsOpen] = useState(false)

  const toggleForm = () => setFormIsOpen(!formIsOpen)

  const topic = useFragment(
    graphql`
      fragment Topic_topic on Topic {
        displayColor
        displayName
        id
        loading
        newlyAdded
        viewerCanUpdate

        displayParentTopics(first: 100) {
          edges {
            node {
              id
              displayName
            }
          }
        }
      }
    `,
    props.topic,
  )

  const showEditButton = !topic.loading && topic.viewerCanUpdate
  const displayParentTopics = liftNodes<ParentTopicType>(topic.displayParentTopics)

  return (
    <Item
      canEdit={topic.viewerCanUpdate}
      className="topicTopicRow Box-row--topic"
      displayColor={topic.displayColor as string}
      formIsOpen={formIsOpen}
      newlyAdded={topic.newlyAdded}
      showEditButton={showEditButton}
      showLink={false}
      title={topic.displayName}
      toggleForm={toggleForm}
      topics={displayParentTopics}
      url={topicPath(topic.id)}
    >
      <Suspense fallback={<div>loading ...</div>}>
        <EditTopic
          isOpen={formIsOpen}
          toggleForm={toggleForm}
          topicId={topic.id}
        />
      </Suspense>
    </Item>
  )
}
