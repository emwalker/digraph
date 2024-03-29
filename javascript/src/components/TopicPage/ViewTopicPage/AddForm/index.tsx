import React from 'react'
import { graphql, useFragment } from 'react-relay'

import {
  AddForm_parentTopic$key,
  AddForm_parentTopic$data as TopicType,
} from '__generated__/AddForm_parentTopic.graphql'
import {
  AddForm_viewer$key, AddForm_viewer$data as ViewerType,
} from '__generated__/AddForm_viewer.graphql'
import AddTopic from './AddTopic'
import AddLink from './AddLink'
import SelectRepository from './SelectRepository'
import './index.css'
import Blankslate from 'components/ui/Blankslate'

type Props = {
  parentTopic: AddForm_parentTopic$key,
  viewer: AddForm_viewer$key,
}

const viewerFragment = graphql`
  fragment AddForm_viewer on User {
    selectedRepoId

    selectedRepo {
      isPrivate
      displayColor
    }

    ...AddLink_viewer
    ...AddTopic_viewer
    ...SelectRepository_viewer
    ...SelectedRepo_viewer
  }
`

const parentTopicFragment = graphql`
  fragment AddForm_parentTopic on Topic {
    id

    ...AddLink_parentTopic
    ...AddTopic_parentTopic

    repoTopics {
      repoId
    }
  }
`

type InnerAddFormProps = {
  isPrivateRepo: boolean,
  viewer: ViewerType,
  topic: TopicType,
}

function InnerAddForm({ isPrivateRepo, viewer, topic }: InnerAddFormProps) {
  const selectedRepoId = viewer.selectedRepoId
  const hasSelectedRepo = !!viewer.selectedRepo
  const topicInSelectedRepo = !!topic.repoTopics
    .find((repoTopic) => repoTopic.repoId == selectedRepoId)
  const canUpsert = hasSelectedRepo && (isPrivateRepo || topicInSelectedRepo)

  if (!selectedRepoId) return null

  if (canUpsert) {
    return (
      <>
        <AddTopic disabled={!hasSelectedRepo} parentTopic={topic} viewer={viewer} />
        <AddLink disabled={!hasSelectedRepo} parentTopic={topic} viewer={viewer} />
      </>
    )
  }

  return (
    <div data-testid="upserts-disabled" >
      <Blankslate>
        <p>This is a private topic, and in order to protect confidentiality,
          subtopics and links cannot be added to it by way of a public repo</p>
      </Blankslate>
    </div >
  )
}

export default function AddForm(props: Props) {
  const viewer = useFragment(viewerFragment, props.viewer)
  const parentTopic = useFragment(parentTopicFragment, props.parentTopic)
  const isPrivateRepo = !!viewer.selectedRepo?.isPrivate

  const selectRepositoryStyle = {
    backgroundColor: isPrivateRepo ?
      viewer.selectedRepo?.displayColor :
      'transparent',
  }

  return (
    <form className="border rounded-1 px-md-2 px-3 mt-3" style={selectRepositoryStyle}>
      <SelectRepository currentTopicId={parentTopic.id} viewer={viewer} />
      <InnerAddForm isPrivateRepo={isPrivateRepo} topic={parentTopic} viewer={viewer} />
    </form>
  )
}
