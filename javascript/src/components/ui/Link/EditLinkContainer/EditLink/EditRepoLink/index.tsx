import React, { FormEvent, useCallback, useState } from 'react'
import { graphql, useFragment } from 'react-relay'

import { liftNodes, TopicOption } from 'components/types'
import Input from 'components/ui/Input'
import { makeUpsertLinkCallback as makeUpsertLinkCallback } from 'mutations/upsertLinkMutation'
import EditParentTopicList, { makeOptions } from 'components/ui/EditParentTopicList'
import DeleteButton from 'components/ui/DeleteButton'
import { EditRepoLink_repoLink$key } from '__generated__/EditRepoLink_repoLink.graphql'
import { EditRepoLink_viewer$key } from '__generated__/EditRepoLink_viewer.graphql'
import { makeDeleteLinkCallback as makeDeleteLinkCallback } from 'mutations/deleteLinkMutation'
import { makeUpdateLinkParentTopicsCallback } from 'mutations/updateLinkParentTopicsMutation'
import { borderColor } from 'components/helpers'

type Props = {
  refetch: Function,
  repoLink: EditRepoLink_repoLink$key,
  viewer: EditRepoLink_viewer$key,
}

const viewerFragment = graphql`
  fragment EditRepoLink_viewer on User {
    selectedRepository {
      id
    }
  }
`

const repoLinkFragment = graphql`
  fragment EditRepoLink_repoLink on RepoLink @argumentDefinitions(
    searchString: {type: "String", defaultValue: null},
  ) {
    displayColor
    linkId
    title
    url

    selectedTopics: parentTopics(first: 1000) {
      edges {
        node {
          value: id
          label: displayName
        }
      }
    }

    availableTopics: availableParentTopics(searchString: $searchString) {
      synonymMatches {
        value: id
        label: displayName
      }
    }
  }
`

export default function EditRepoLink({ refetch, ...rest }: Props) {
  const viewer = useFragment(viewerFragment, rest.viewer)
  const repoLink = useFragment(repoLinkFragment, rest.repoLink)
  const [title, setTitle] = useState(repoLink.title)

  const selectedRepoId = viewer.selectedRepository?.id || null
  const { linkId, url } = repoLink

  const onSave = makeUpsertLinkCallback({ selectedRepoId, linkId, title, url })
  const onDelete = makeDeleteLinkCallback({ selectedRepoId, linkId })
  const selectedTopics = makeOptions(liftNodes(repoLink.selectedTopics))
  const updateTopics = makeUpdateLinkParentTopicsCallback({ selectedRepoId, linkId })
  const inputId = `edit-link-title-${linkId}`

  const updateTitle = useCallback((event: FormEvent<HTMLTextAreaElement>) => {
    setTitle(event.currentTarget.value)
  }, [setTitle])

  const loadOptions = useCallback((searchString: string) => {
    const promise = new Promise<readonly TopicOption[]>((resolve) => {
      const variables = {
        count: 40,
        searchString,
      }

      refetch(variables, null, () => {
        const { availableTopics } = repoLink
        const options = availableTopics ? makeOptions(availableTopics.synonymMatches) : []
        resolve(options)
      })
    })

    return promise
  }, [refetch, repoLink])

  return (
    <li className="Box-row" style={{ borderColor: borderColor(repoLink.displayColor) }}>
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

      <EditParentTopicList
        loadOptions={loadOptions}
        selectedTopics={selectedTopics}
        updateTopics={updateTopics}
      />
    </li>
  )
}
