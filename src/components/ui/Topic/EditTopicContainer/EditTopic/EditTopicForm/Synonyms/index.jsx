// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { Relay } from 'components/types'
import updateSynonymsMutation from 'mutations/updateSynonymsMutation'
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
  synonyms: $ReadOnlyArray<SynonymType>,
}

class Synonyms extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      locale: 'en',
      name: '',
      synonyms: props.topic.synonyms,
    }
  }

  onLocaleChange = (event: SyntheticInputEvent<HTMLButtonElement>) => {
    this.setState({ locale: event.target.value })
  }

  onNameChange = (event: SyntheticInputEvent<HTMLButtonElement>) => {
    this.setState({ name: event.target.value })
  }

  onAdd = () => {
    const update = copySynonyms(this.state.synonyms)
    const synonym = ({ name: this.state.name, locale: this.state.locale }: any)
    update.push(synonym)
    this.updateSynonyms(update)
  }

  onDelete = (idx: number) => {
    // eslint-disable-next-line no-alert
    if (!window.confirm('Are you sure you want to delete this synonym?')) return

    const update = copySynonyms(this.state.synonyms)
    update.splice(idx, 1)
    this.updateSynonyms((update: any))
  }

  get synonyms(): $ReadOnlyArray<SynonymType> {
    return this.props.topic.synonyms
  }

  updateSynonyms = (synonyms: $ReadOnlyArray<SynonymType>) => {
    this.setState({ synonyms, locale: 'en', name: '' }, () => {
      updateSynonymsMutation(
        this.props.relay.environment,
        [],
        { topicId: this.props.topic.id, synonyms },
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

  renderAddButton = () => (
    <button type="button" onClick={this.onAdd} className="btn col-1">
      Add
    </button>
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
      <div className="clearfix">
        <input
          id="names-and-synonyms"
          className="form-control col-10 mr-2"
          onChange={this.onNameChange}
          value={this.state.name}
        />

        <select onChange={this.onLocaleChange} className="form-select col-1 mr-2">
          <option>en</option>
          <option>es</option>
          <option>fr</option>
        </select>

        { this.props.topic.viewerCanUpdate && this.renderAddButton() }
      </div>
    </dl>
  )
}

export const UnwrappedSynonyms = Synonyms

export default createFragmentContainer(Synonyms, {
  topic: graphql`
    fragment Synonyms_topic on Topic {
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
