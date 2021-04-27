import React, { Component, FormEvent, ChangeEvent } from 'react'
import { createFragmentContainer, graphql, RelayProp } from 'react-relay'

import updateSynonymsMutation, { Input } from 'mutations/updateSynonymsMutation'
import { Synonyms_topic as TopicType } from '__generated__/Synonyms_topic.graphql'
import { SynonymType } from 'components/types'
import SynonymList from './SynonymList'
import copySynonyms from './copySynonyms'

type Props = {
  relay: RelayProp,
  topic: TopicType,
}

type State = {
  locale: string,
  name: string,
}

class Synonyms extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      locale: 'en',
      name: '',
    }
  }

  onLocaleChange = (event: FormEvent<HTMLSelectElement>) => {
    this.setState({ locale: event.currentTarget.value })
  }

  onNameChange = (event: ChangeEvent<HTMLInputElement>) => {
    this.setState({ name: event.currentTarget.value })
  }

  onAdd = () => {
    const update = copySynonyms(this.synonyms)
    const synonym = { name: this.state.name, locale: this.state.locale }
    update.push(synonym)
    this.updateSynonyms(update)
  }

  onDelete = (position: number) => {
    // eslint-disable-next-line no-alert
    if (!window.confirm('Are you sure you want to delete this synonym?')) return

    const update = copySynonyms(this.synonyms)
    update.splice(position, 1)
    this.updateSynonyms(update)
  }

  get synonyms() {
    return this.props.topic.synonyms
  }

  optimisticResponse = (synonyms: SynonymType[]) => (
    {
      updateSynonyms: {
        alerts: [],
        clientMutationId: null,
        topic: {
          ...this.props.topic,
          synonyms,
        },
      },
    }
  )

  updateSynonyms = (synonyms: SynonymType[]) => {
    const input: Input = { topicId: this.props.topic.id, synonyms }

    this.setState({ locale: 'en', name: '' }, () => {
      updateSynonymsMutation(
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
      onUpdate={this.updateSynonyms}
      synonyms={this.synonyms}
    />
  )

  renderAddForm = () => (
    <div className="clearfix">
      <input
        id="names-and-synonyms"
        className="form-control col-12 col-lg-10 mr-2"
        onChange={this.onNameChange}
        value={this.state.name}
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
        { this.renderSynonyms() }
      </ul>
      { this.props.topic.viewerCanUpdate && this.renderAddForm() }
    </dl>
  )
}

export const UnwrappedSynonyms = Synonyms

export default createFragmentContainer(Synonyms, {
  topic: graphql`
    fragment Synonyms_topic on Topic {
      displayName: name
      id
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
