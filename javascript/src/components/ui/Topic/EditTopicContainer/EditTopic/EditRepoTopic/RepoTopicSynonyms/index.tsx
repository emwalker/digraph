import React, {
  MouseEventHandler, ChangeEventHandler, FormEvent, ChangeEvent, useCallback, useState,
} from 'react'
import { graphql, useFragment, useMutation, useRelayEnvironment } from 'react-relay'

import updateSynonymsQuery from 'mutations/updateTopicSynonymsMutation'
import {
  updateTopicSynonymsMutation,
} from '__generated__/updateTopicSynonymsMutation.graphql'
import {
  RepoTopicSynonyms_repoTopic$key,
  RepoTopicSynonyms_repoTopic$data as RepoTopicType,
} from '__generated__/RepoTopicSynonyms_repoTopic.graphql'
import { RepoTopicSynonyms_viewer$key } from '__generated__/RepoTopicSynonyms_viewer.graphql'
import { SynonymType } from 'components/types'
import SynonymList from './SynonymList'
import copySynonyms from './copySynonyms'

type Props = {
  repoTopic: RepoTopicSynonyms_repoTopic$key,
  viewer: RepoTopicSynonyms_viewer$key,
}

function displayName(synonyms: SynonymType[]) {
  if (synonyms.length > 0) {
    for (const synonym of synonyms) {
      if (synonym.locale !== 'en') // FIXME
        continue
      return synonym.name
    }
    return synonyms[0].name
  }

  return 'Missing name'
}

function optimisticResponse(repoTopic: RepoTopicType, synonymUpdate: SynonymType[]) {
  return {
    updateTopicSynonyms: {
      clientMutationId: null,
      alerts: [],
      updatedTopic: {
        id: repoTopic.topicId,
        displayName: displayName(synonymUpdate),
      },
      updatedRepoTopic: {
        ...repoTopic,
        synonyms: synonymUpdate,
      },
    },
  }
}

const renderSynonyms = (
  synonyms: readonly SynonymType[],
  viewerCanUpdate: boolean,
  onDelete: (position: number) => void,
  updateTopicSynonyms: (synonyms: SynonymType[]) => void,
) => {
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

const renderAddForm = (
  inputName: string,
  onNameChange: ChangeEventHandler<HTMLInputElement>,
  onLocaleChange: ChangeEventHandler<HTMLSelectElement>,
  onAdd: MouseEventHandler<HTMLButtonElement>,
) => (
  <div className="clearfix">
    <input
      id="names-and-synonyms"
      style={{ width: '70%' }}
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

const repoTopicFragment = graphql`
  fragment RepoTopicSynonyms_repoTopic on RepoTopic {
    id
    topicId
    viewerCanDeleteSynonyms
    viewerCanUpdate

    repo {
      id
    }

    synonyms {
      name
      locale
    }
  }
`

const viewerFragment = graphql`
  fragment RepoTopicSynonyms_viewer on User {
    selectedRepository {
      id
    }
  }
`

export default function RepoTopicSynonyms(props: Props) {
  const repoTopic = useFragment(repoTopicFragment, props.repoTopic)
  const viewer = useFragment(viewerFragment, props.viewer)
  const updateSynonyms = useMutation<updateTopicSynonymsMutation>(updateSynonymsQuery)[0]

  const [inputName, setInputName] = useState('')
  const [inputLocale, setInputLocale] = useState('en')

  const repoId = viewer.selectedRepository?.id || null
  const synonyms = repoTopic?.synonyms || []
  const environment = useRelayEnvironment()

  const updateTopicSynonyms = useCallback((synonymUpdate: SynonymType[]) => {
    if (!repoTopic) return null

    if (!repoId) {
      console.log('no repo selected')
      return
    }

    updateSynonyms({
      variables: {
        input: { repoId, topicId: repoTopic.topicId, synonyms: synonymUpdate },
      },
      optimisticResponse: optimisticResponse(repoTopic, synonymUpdate),
    })
    setInputName('')
  }, [repoId, repoTopic, environment, setInputName, updateSynonyms])

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
        {renderSynonyms(synonyms, repoTopic.viewerCanUpdate, onDelete, updateTopicSynonyms)}
      </ul>

      {repoTopic.viewerCanUpdate && renderAddForm(inputName, onNameChange, onLocaleChange, onAdd)}
    </dl>
  )
}
