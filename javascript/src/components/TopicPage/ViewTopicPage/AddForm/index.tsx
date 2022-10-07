import React from 'react'
import { graphql, useFragment } from 'react-relay'

import { AddForm_topic$key } from '__generated__/AddForm_topic.graphql'
import { AddForm_viewer$key } from '__generated__/AddForm_viewer.graphql'
import AddTopic from './AddTopic'
import AddLink from './AddLink'
import SelectRepository from './SelectRepository'
import './index.css'

type Props = {
  topic: AddForm_topic$key,
  viewer: AddForm_viewer$key,
}

export default function AddForm(props: Props) {
  const viewer = useFragment(
    graphql`
      fragment AddForm_viewer on User {
        selectedRepo {
          isPrivate
          displayColor
        }

        ...AddLink_viewer
        ...AddTopic_viewer
        ...SelectRepository_viewer
        ...SelectedRepo_viewer
      }
    `,
    props.viewer,
  )

  const topic = useFragment(
    graphql`
      fragment AddForm_topic on Topic {
        ...AddLink_parentTopic
        ...AddTopic_parentTopic
      }
    `,
    props.topic,
  )

  const isPrivateRepo = !!viewer.selectedRepo?.isPrivate
  const repoSelected = !!viewer.selectedRepo
  const selectRepositoryStyle = {
    backgroundColor: isPrivateRepo ?
      viewer.selectedRepo?.displayColor :
      'transparent',
  }

  return (
    <form className="border rounded-1 px-md-2 px-3 mt-3" style={selectRepositoryStyle}>
      <SelectRepository viewer={viewer} />
      {repoSelected && (
        <>
          <AddTopic disabled={!repoSelected} parentTopic={topic} viewer={viewer} />
          <AddLink disabled={!repoSelected} parentTopic={topic} viewer={viewer} />
        </>
      )}
    </form>
  )
}
