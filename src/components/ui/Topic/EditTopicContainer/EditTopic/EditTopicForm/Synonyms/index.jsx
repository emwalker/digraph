// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Relay } from 'components/types'
import updateSynonymsMutation, { type Input } from 'mutations/updateSynonymsMutation'
import SynonymList from './SynonymList'
import { type Topic, type Synonym as SynonymType } from './types'
import copySynonyms from './copySynonyms'

type Props = {
  relay: Relay,
  topic: Topic,
}

type State = {
  locale: string,
  name: string,
}

type Response = {
}

class Synonyms extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      locale: 'en',
      name: '',
    }
  }

  onLocaleChange = (event: SyntheticInputEvent<HTMLButtonElement>) => {
    this.setState({ locale: event.target.value })
  }

  onNameChange = (event: SyntheticInputEvent<HTMLButtonElement>) => {
    this.setState({ name: event.target.value })
  }

  onAdd = () => {
    const update = copySynonyms(this.synonyms)
    const synonym = ({ name: this.state.name, locale: this.state.locale }: any)
    update.push(synonym)
    this.updateSynonyms(update)
  }

  onDelete = (position: number) => {
    // eslint-disable-next-line no-alert
    if (!window.confirm('Are you sure you want to delete this synonym?')) return

    const update = copySynonyms(this.synonyms)
    update.splice(position, 1)
    this.updateSynonyms((update: any))
  }

  get synonyms(): $ReadOnlyArray<SynonymType> {
    return this.props.topic.synonyms
  }

  optimisticResponse = (synonyms: $ReadOnlyArray<SynonymType>): Response => (
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

  updateSynonyms = (synonyms: $ReadOnlyArray<SynonymType>) => {
    // $FlowFixMe
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
      onAdd={this.onAdd}
      onDelete={this.onDelete}
      onUpdate={this.updateSynonyms}
      relay={this.props.relay}
      synonyms={this.synonyms}
      topic={this.props.topic}
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
          <option>es</option>
          <option>fr</option>
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
