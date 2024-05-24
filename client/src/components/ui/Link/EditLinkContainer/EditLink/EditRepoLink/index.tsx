import React, { FormEvent, useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import Input from 'components/ui/Input'
import { makeUpsertLinkCallback } from 'mutations/upsertLinkMutation'
import DeleteButton from 'components/ui/DeleteButton'
import { EditRepoLink_repoLink$key } from '__generated__/EditRepoLink_repoLink.graphql'
import { EditRepoLink_viewer$key } from '__generated__/EditRepoLink_viewer.graphql'
import { makeDeleteLinkCallback as makeDeleteLinkCallback } from 'mutations/deleteLinkMutation'
import ParentTopics from './RepoLinkParentTopics'

type Props = {
  repoLink: EditRepoLink_repoLink$key,
  viewer: EditRepoLink_viewer$key,
}

const viewerFragment = graphql`
  fragment EditRepoLink_viewer on User {
    id
    selectedRepoId
  }
`

const repoLinkFragment = graphql`
  fragment EditRepoLink_repoLink on RepoLink {
    displayColor
    linkId

    details {
      title
      url
    }

    ...RepoLinkParentTopics_repoLink
  }
`

export default function EditRepoLink(props: Props) {
  const viewer = useFragment(viewerFragment, props.viewer)
  const repoLink = useFragment(repoLinkFragment, props.repoLink)
  const { linkId, details } = repoLink
  const [title, setTitle] = useState(details?.title)

  const selectedRepoId = viewer.selectedRepoId
  const viewerId = viewer.id
  const url = details?.url

  const onSave = makeUpsertLinkCallback({ selectedRepoId, linkId, title, url: url || null })
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
    <li className="Box-row edit-repo-link">
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
              placeholder="inherited"
            />
          </dd>
        </dl>
      </div>

      <Input
        className="col-12"
        id={`edit-link-url-${linkId}`}
        label="Url"
        disabled={true}
        placeholder="inherited"
        value={url}
      />

      <ParentTopics
        repoLink={repoLink}
        selectedRepoId={selectedRepoId}
        viewerId={viewerId}
      />

      <div className="pb-1">
        <button type="submit" onClick={onSave} className="btn-primary">Save</button>
        <DeleteButton
          className="float-right"
          onDelete={onDelete}
        />
      </div>
    </li>
  )
}
