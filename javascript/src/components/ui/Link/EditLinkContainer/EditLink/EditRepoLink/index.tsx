import React, { FormEvent, useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import Input from 'components/ui/Input'
import { makeUpsertLinkCallback as makeUpsertLinkCallback } from 'mutations/upsertLinkMutation'
import DeleteButton from 'components/ui/DeleteButton'
import { EditRepoLink_repoLink$key } from '__generated__/EditRepoLink_repoLink.graphql'
import { EditRepoLink_viewer$key } from '__generated__/EditRepoLink_viewer.graphql'
import { makeDeleteLinkCallback as makeDeleteLinkCallback } from 'mutations/deleteLinkMutation'
import ParentTopics from './EditRepoLinkParentTopics'

type Props = {
  repoLink: EditRepoLink_repoLink$key,
  viewer: EditRepoLink_viewer$key,
}

const viewerFragment = graphql`
  fragment EditRepoLink_viewer on User {
    id
    selectedRepository {
      id
    }
  }
`

const repoLinkFragment = graphql`
  fragment EditRepoLink_repoLink on RepoLink {
    displayColor
    linkId
    title
    url

    ...EditRepoLinkParentTopics_repoLink
  }
`

export default function EditRepoLink(props: Props) {
  const viewer = useFragment(viewerFragment, props.viewer)
  const repoLink = useFragment(repoLinkFragment, props.repoLink)
  const [title, setTitle] = useState(repoLink.title)

  const selectedRepoId = viewer.selectedRepository?.id || null
  const viewerId = viewer.id
  const { linkId, url } = repoLink

  const onSave = makeUpsertLinkCallback({ selectedRepoId, linkId, title, url })
  const onDelete = makeDeleteLinkCallback({ selectedRepoId, linkId })
  const inputId = `edit-link-title-${linkId}`

  const updateTitle = useCallback((event: FormEvent<HTMLTextAreaElement>) => {
    setTitle(event.currentTarget.value)
  }, [setTitle])

  if (!selectedRepoId) {
    console.log('attempt to edit repo link without a selected repo')
    return null
  }

  if (!viewerId) {
    console.log('no viewer')
    return null
  }

  return (
    <li className="Box-row">
      <div className="col-12">
        <dl className="form-group">
          <dt>
            <label htmlFor={inputId}>Page title</label>
          </dt>
          <dd>
            <textarea
              className="form-control"
              defaultValue={title || ''}
              id={inputId}
              onChange={updateTitle}
            />
          </dd>
        </dl>
      </div>

      <Input
        className="col-12"
        id={`edit-link-url-${linkId}`}
        label="Url"
        disabled={true}
        value={url}
      />

      <div>
        <button type="submit" onClick={onSave} className="btn-primary">Save</button>
        <DeleteButton
          className="float-right"
          onDelete={onDelete}
        />
      </div>

      <ParentTopics
        viewerId={viewerId}
        selectedRepoId={selectedRepoId}
        repoLink={repoLink}
      />
    </li>
  )
}
