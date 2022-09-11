import React, {
  MouseEventHandler, ChangeEventHandler, FormEvent, ChangeEvent, useCallback, useState,
} from 'react'
import { graphql, useFragment, useRelayEnvironment } from 'react-relay'

import updateTopicSynonymsMutation, { Input } from 'mutations/updateTopicSynonymsMutation'
import {
  Synonyms_repoTopic$key,
  Synonyms_repoTopic$data as RepoTopicType,
} from '__generated__/Synonyms_repoTopic.graphql'
import { Synonyms_viewer$key } from '__generated__/Synonyms_viewer.graphql'
import { SynonymType } from 'components/types'
import SynonymList from './SynonymList'
import copySynonyms from './copySynonyms'

type Props = {
  repoTopic: Synonyms_repoTopic$key,
  viewer: Synonyms_viewer$key,
}

function displayName(synonyms: SynonymType[]) {
  if (synonyms.length > 0) {
    for (const synonym of synonyms) {
      if (synonym.locale != 'en')
        continue
      return synonym.name
    }
    return synonyms[0].name
  }

  return 'Missing name'
}

function optimisticResponse(repoTopic: RepoTopicType, synonyms: SynonymType[]) {
  return {
    updateTopicSynonyms: {
      alerts: [],
      clientMutationId: null,
      ...repoTopic,
      synonyms,
    },
  }
}

const renderSynonyms = (
  repoTopic: RepoTopicType | null,
  onDelete: Function,
  updateTopicSynonyms: Function,
) => {
  if (!repoTopic) return null

  return (
    <SynonymList
      canUpdate={repoTopic.viewerCanUpdate}
      onDelete={onDelete}
      onUpdate={updateTopicSynonyms}
      synonyms={repoTopic?.synonyms}
    />
  )
}

const renderAddForm = (
  inputName: string,
  onNameChange: ChangeEventHandler<HTMLInputElement>,
  onLocaleChange: ChangeEventHandler<HTMLSelectElement>,
  onAdd: MouseEventHandler<HTMLButtonElement>,
) => (
  <div className="clearfix">
    <input
      id="names-and-synonyms"
      className="form-control col-12 col-lg-10 mr-2"
      onChange={onNameChange}
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

      <button type="button" onClick={onAdd} className="btn">
        Add
      </button>
    </div>
  </div>
)

export default function Synonyms(props: Props) {
  const repoTopic = useFragment(
    graphql`
      fragment Synonyms_repoTopic on RepoTopic {
        topicId
        displayName
        viewerCanDeleteSynonyms
        viewerCanUpdate

        synonyms {
          name
          locale

          ...Synonym_synonym
        }
      }
    `,
    props.repoTopic,
  )

  const viewer = useFragment(
    graphql`
      fragment Synonyms_viewer on User {
        selectedRepository {
          id
        }
      }
    `,
    props.viewer,
  )

  const [inputName, setInputName] = useState('')
  const [inputLocale, setInputLocale] = useState('en')

  const repoId = viewer.selectedRepository?.id
  const synonyms = repoTopic?.synonyms || []
  const environment = useRelayEnvironment()

  const updateTopicSynonyms = useCallback((update: SynonymType[]) => {
    if (!repoTopic) return null

    if (!repoId) {
      console.log('no repo selected')
      return
    }
  
    const input: Input = { repoId, topicId: repoTopic.topicId, synonyms: update }
    const response = optimisticResponse(repoTopic, update)
    console.log('input:', input)
  
    updateTopicSynonymsMutation(
      environment,
      input,
      { optimisticResponse: response },
    )
    setInputName('')
  }, [
    repoId, repoTopic, environment, setInputName, updateTopicSynonymsMutation, optimisticResponse,
    useRelayEnvironment,
  ])

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
    updateTopicSynonyms(update)
  }, [inputName, repoTopic, synonyms, copySynonyms, updateTopicSynonyms])

  const onDelete = useCallback((position: number) => {
    // eslint-disable-next-line no-alert
    if (!window.confirm('Are you sure you want to delete this synonym?')) return

    const update = copySynonyms(synonyms)
    update.splice(position, 1)
    updateTopicSynonyms(update)
  }, [copySynonyms, updateTopicSynonyms])

  return (
    <dl className="form-group">
      <label
        htmlFor="names-and-synonyms"
      >
        Names and synonyms
      </label>
      <ul className="Box list-style-none mt-1 mb-2">
        {renderSynonyms(repoTopic, onDelete, updateTopicSynonyms)}
      </ul>

      {repoTopic.viewerCanUpdate && renderAddForm(inputName, onNameChange, onLocaleChange, onAdd)}
    </dl>
  )
}
