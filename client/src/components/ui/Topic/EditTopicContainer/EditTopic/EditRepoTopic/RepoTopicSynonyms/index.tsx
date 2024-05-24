import React, {
  MouseEventHandler, ChangeEventHandler, FormEvent, ChangeEvent, useCallback, useState,
} from 'react'
import { graphql, useFragment } from 'react-relay'

import { makeUpdateTopicSynonymsCallback } from 'mutations/updateTopicSynonymsMutation'
import {
  RepoTopicSynonyms_repoTopic$key,
} from '__generated__/RepoTopicSynonyms_repoTopic.graphql'
import { RepoTopicSynonyms_viewer$key } from '__generated__/RepoTopicSynonyms_viewer.graphql'
import { SynonymType } from 'components/types'
import SynonymList from './SynonymList'
import copySynonyms from './copySynonyms'

type Props = {
  repoTopic: RepoTopicSynonyms_repoTopic$key,
  viewer: RepoTopicSynonyms_viewer$key,
}

type SynonymListOuterProps = {
  synonyms: readonly SynonymType[],
  viewerCanUpdate: boolean,
  onDelete: (position: number) => void,
  updateTopicSynonyms: (synonyms: SynonymType[]) => void,
}

function SynonymListOuter({
  synonyms, viewerCanUpdate, onDelete, updateTopicSynonyms,
}: SynonymListOuterProps) {
  if (synonyms.length === 0)
    return <div className="blankslate"><p>There are no synonyms</p></div>

  return (
    <SynonymList
      canUpdate={viewerCanUpdate}
      onDelete={onDelete}
      onUpdate={updateTopicSynonyms}
      synonyms={synonyms}
    />
  )
}

type AddFormProps = {
  inputName: string,
  onNameChange: ChangeEventHandler<HTMLInputElement>,
  onLocaleChange: ChangeEventHandler<HTMLSelectElement>,
  onAdd: MouseEventHandler<HTMLButtonElement>,
}

function AddForm({ inputName, onNameChange, onLocaleChange, onAdd }: AddFormProps) {
  const addDisabled = inputName.trim() === ''

  return (
    <div className="clearfix">
      <input
        className="form-control col-12 col-lg-10 mr-2"
        data-testid="synonym-input"
        id="names-and-synonyms"
        onChange={onNameChange}
        style={{ width: '70%' }}
        value={inputName}
      />

      <div className="col-12 col-lg-3 mt-2 d-inline-block">
        <select onChange={onLocaleChange} className="form-select mr-2">
          <option>en</option>
          <option>ar</option>
          <option>de</option>
          <option>el</option>
          <option>es</option>
          <option>fa</option>
          <option>fi</option>
          <option>fr</option>
          <option>hi</option>
          <option>it</option>
          <option>ja</option>
          <option>ji</option>
          <option>ko</option>
          <option>la</option>
          <option>nl</option>
          <option>no</option>
          <option>pt</option>
          <option>ru</option>
          <option>sv</option>
          <option>tr</option>
          <option>uk</option>
          <option>zh</option>
        </select>

        <button
          className="btn"
          data-testid="add-button"
          disabled={addDisabled}
          onClick={onAdd}
          type="button"
        >
          Add
        </button>
      </div>
    </div>
  )
}

const repoTopicFragment = graphql`
  fragment RepoTopicSynonyms_repoTopic on RepoTopic {
    id
    repoId
    timerangePrefix
    topicId
    viewerCanDeleteSynonyms
    viewerCanUpdate

    details {
      synonyms {
        name
        locale
      }
    }
  }
`

const viewerFragment = graphql`
  fragment RepoTopicSynonyms_viewer on User {
    selectedRepoId
  }
`

export default function RepoTopicSynonyms(props: Props) {
  const repoTopic = useFragment(repoTopicFragment, props.repoTopic)
  const viewer = useFragment(viewerFragment, props.viewer)
  const selectedRepoId = viewer.selectedRepoId

  const [inputName, setInputName] = useState('')
  const [inputLocale, setInputLocale] = useState('en')

  const updateSynonyms = makeUpdateTopicSynonymsCallback({
    selectedRepoId, repoTopic, setInputName,
  })

  const synonyms = repoTopic.details?.synonyms || []

  const onNameChange = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    setInputName(event.currentTarget.value)
  }, [setInputName])

  const onLocaleChange = useCallback((event: FormEvent<HTMLSelectElement>) => {
    setInputLocale(event.currentTarget.value)
  }, [setInputLocale])

  const onAdd = useCallback(() => {
    const update = copySynonyms(synonyms)
    const synonym = { name: inputName, locale: inputLocale }
    update.push(synonym)
    updateSynonyms(update)
  }, [inputName, repoTopic, synonyms, copySynonyms, updateSynonyms])

  const onDelete = useCallback((position: number) => {
    if (!window.confirm('Are you sure you want to delete this synonym?')) return

    const update = copySynonyms(synonyms)
    update.splice(position, 1)
    updateSynonyms(update)
  }, [copySynonyms, updateSynonyms])

  return (
    <dl className="form-group">
      <label htmlFor="names-and-synonyms">Names and synonyms</label>

      <ul className="Box list-style-none mt-1 mb-2">
        <SynonymListOuter
          onDelete={onDelete}
          synonyms={synonyms}
          updateTopicSynonyms={updateSynonyms}
          viewerCanUpdate={repoTopic.viewerCanUpdate}
        />
      </ul>

      {repoTopic.viewerCanUpdate && (
        <AddForm
          inputName={inputName}
          onAdd={onAdd}
          onLocaleChange={onLocaleChange}
          onNameChange={onNameChange}
        />
      )}
    </dl>
  )
}
