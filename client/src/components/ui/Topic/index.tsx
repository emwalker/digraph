import React, { useState, useCallback } from 'react'
import { graphql, useFragment } from 'react-relay'

import { topicPath } from 'components/helpers'
import { NodeTypeOf, liftNodes, Color } from 'components/types'
import Item from '../Item'
import EditTopicLoader from './EditTopicLoader'
import { Topic_topic$data, Topic_topic$key } from '__generated__/Topic_topic.graphql'
import { Topic_viewer$key } from '__generated__/Topic_viewer.graphql'

type ParentTopicType = NodeTypeOf<Topic_topic$data['displayParentTopics']>

type Props = {
  topic: Topic_topic$key,
  viewer: Topic_viewer$key,
}

const viewerFragment = graphql`
  fragment Topic_viewer on User {
    id
    selectedRepoId
  }
`

const topicFragment = graphql`
  fragment Topic_topic on Topic {
    displayName
    id
    loading
    newlyAdded
    viewerCanUpdate
    showRepoOwnership

    repoTopics {
      inWikiRepo
      displayColor
    }

    displayParentTopics(first: 100) {
      edges {
        node {
          id
          displayName
        }
      }
    }
  }
`

export default function Topic(props: Props) {
  const topic = useFragment(topicFragment, props.topic)
  const viewer = useFragment(viewerFragment, props.viewer)
  const [formIsOpen, setFormIsOpen] = useState(false)

  const toggleForm = useCallback(() => setFormIsOpen(!formIsOpen), [setFormIsOpen, formIsOpen])
  const showEditButton = !topic.loading && topic.viewerCanUpdate
  const displayParentTopics = liftNodes<ParentTopicType>(topic.displayParentTopics)
  const repoColors = (topic.repoTopics || []) .map((repoTopic) => repoTopic.displayColor as Color)
  const canEdit = !!(topic.viewerCanUpdate && viewer.selectedRepoId)

  return (
    <Item
      canEdit={canEdit}
      className="topicTopicRow Box-row--topic"
      formIsOpen={formIsOpen}
      newlyAdded={topic.newlyAdded}
      repoColors={repoColors}
      showEditButton={showEditButton}
      showLink={false}
      showRepoOwnership={topic.showRepoOwnership}
      title={topic.displayName}
      toggleForm={toggleForm}
      topics={displayParentTopics}
      url={topicPath(topic.id)}
    >
      {formIsOpen && viewer.id && (
        <EditTopicLoader
          topicId={topic.id}
          viewerId={viewer.id}
        />
      )}
    </Item>
  )
}
