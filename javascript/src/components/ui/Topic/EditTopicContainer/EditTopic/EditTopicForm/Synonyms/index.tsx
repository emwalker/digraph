import React, { Component, FormEvent, ChangeEvent } from 'react'
import { createFragmentContainer, graphql, RelayProp } from 'react-relay'

import updateTopicSynonymsMutation, { Input } from 'mutations/updateTopicSynonymsMutation'
import { Synonyms_topic as TopicType } from '__generated__/Synonyms_topic.graphql'
import { SynonymType } from 'components/types'
import SynonymList from './SynonymList'
import copySynonyms from './copySynonyms'

type Props = {
  relay: RelayProp,
  topic: TopicType,
}

type State = {
  inputLocale: string,
  inputName: string,
}

const displayName = (synonyms: SynonymType[]) => {
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

class Synonyms extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      inputLocale: 'en',
      inputName: '',
    }
  }

  onLocaleChange = (event: FormEvent<HTMLSelectElement>) => {
    this.setState({ inputLocale: event.currentTarget.value })
  }

  onNameChange = (event: ChangeEvent<HTMLInputElement>) => {
    this.setState({ inputName: event.currentTarget.value })
  }

  onAdd = () => {
    const update = copySynonyms(this.synonyms)
    const synonym = { name: this.state.inputName, locale: this.state.inputLocale }
    update.push(synonym)
    this.updateTopicSynonyms(update)
  }

  onDelete = (position: number) => {
    // eslint-disable-next-line no-alert
    if (!window.confirm('Are you sure you want to delete this synonym?')) return

    const update = copySynonyms(this.synonyms)
    update.splice(position, 1)
    this.updateTopicSynonyms(update)
  }

  get synonyms() {
    return this.props.topic.synonyms
  }

  optimisticResponse = (synonyms: SynonymType[]) => {
    return {
      updateTopicSynonyms: {
        alerts: [],
        clientMutationId: null,
        topic: {
          ...this.props.topic,
          displayName: displayName(synonyms),
          synonyms,
        },
      },
    }
  }

  updateTopicSynonyms = (synonyms: SynonymType[]) => {
    const input: Input = { topicPath: this.props.topic.path, synonyms }

    this.setState({ inputName: '' }, () => {
      updateTopicSynonymsMutation(
        this.props.relay.environment,
        input,
        { optimisticResponse: this.optimisticResponse(synonyms) },
      )
    })
  }

  renderSynonyms = () => (
    <SynonymList
      canUpdate={this.props.topic.viewerCanUpdate}
      onDelete={this.onDelete}
      onUpdate={this.updateTopicSynonyms}
      synonyms={this.synonyms}
    />
  )

  renderAddForm = () => (
    <div className="clearfix">
      <input
        id="names-and-synonyms"
        className="form-control col-12 col-lg-10 mr-2"
        onChange={this.onNameChange}
        value={this.state.inputName}
      />

      <div className="col-12 col-lg-3 mt-2 d-inline-block">
        <select onChange={this.onLocaleChange} className="form-select mr-2">
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

        <button type="button" onClick={this.onAdd} className="btn">
          Add
        </button>
      </div>
    </div>
  )

  render = () => (
    <dl className="form-group">
      <label
        htmlFor="names-and-synonyms"
      >
        Names and synonyms
      </label>
      <ul className="Box list-style-none mt-1 mb-2">
        {this.renderSynonyms()}
      </ul>
      {this.props.topic.viewerCanUpdate && this.renderAddForm()}
    </dl>
  )
}

export const UnwrappedSynonyms = Synonyms

export default createFragmentContainer(Synonyms, {
  topic: graphql`
    fragment Synonyms_topic on Topic {
      id
      displayName: name
      path
      viewerCanDeleteSynonyms
      viewerCanUpdate

      synonyms {
        name
        locale

        ...Synonym_synonym
      }
    }
  `,
})
