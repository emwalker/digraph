// @flow
import React, { Component } from 'react'
import { createFragmentContainer, graphql } from 'react-relay'

import type { CollectionNode, Relay } from 'components/types'
import addSynonymMutation from 'mutations/addSynonymMutation'
import deleteSynonymMutation from 'mutations/deleteSynonymMutation'
import { liftNodes } from 'utils'
import type { Synonyms_topic as Topic } from './__generated__/Synonyms_topic.graphql'
import Synonym from './Synonym'

/* eslint jsx-a11y/label-has-for: 0 */
/* eslint no-restricted-globals: 0 */

type SynonymType = CollectionNode<$PropertyType<Topic, 'synonyms'>>

type Props = {
  relay: Relay,
  topic: Topic,
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

  onDelete = (synonym: SynonymType) => {
    // eslint-disable-next-line no-alert
    if (!confirm('Are you sure you want to delete this synonym?'))
      return

    deleteSynonymMutation(
      this.props.relay.environment,
      this.deleteConfigs,
      { synonymId: synonym.id },
    )
  }

  onLocaleChange = (event: SyntheticInputEvent<HTMLButtonElement>) => {
    this.setState({ locale: event.target.value })
  }

  onNameChange = (event: SyntheticInputEvent<HTMLButtonElement>) => {
    this.setState({ name: event.target.value })
  }

  get synonyms(): SynonymType[] {
    return liftNodes(this.props.topic.synonyms)
  }

  get addConfigs(): Object[] {
    return [
      {
        type: 'RANGE_ADD',
        parentID: this.props.topic.id,
        connectionInfo: [
          {
            key: 'Synonyms_synonyms',
            rangeBehavior: 'append',
          },
        ],
        edgeName: 'synonymEdge',
      },
    ]
  }

  // eslint-disable-next-line class-methods-use-this
  get deleteConfigs(): Object[] {
    return [
      {
        type: 'NODE_DELETE',
        deletedIDFieldName: 'deletedSynonymId',
      },
    ]
  }

  deleteFn = () => (
    this.props.topic.viewerCanDeleteSynonym
      ? this.onDelete
      : null
  )

  addSynonym = () => {
    addSynonymMutation(
      this.props.relay.environment,
      this.addConfigs,
      { topicId: this.props.topic.id, name: this.state.name, locale: this.state.locale },
    )
    this.setState({ name: '', locale: 'en' })
  }

  renderAddButton = () => (
    <button type="button" onClick={this.addSynonym} className="btn col-1">
      Add
    </button>
  )

  render = () => (
    <dl className="form-group">
      <label htmlFor="names-and-synonyms">Names and synonyms</label>
      <ul className="Box list-style-none mt-1 mb-2">
        {this.synonyms.map(synonym => (
          <Synonym onDelete={this.deleteFn()} key={synonym.id} synonym={synonym} />
        ))}
      </ul>
      <div id="names-and-synonym" className="clearfix">
        <input
          className="form-control col-10 mr-2"
          onChange={this.onNameChange}
          value={this.state.name}
        />

        <select onChange={this.onLocaleChange} className="form-select col-1 mr-2">
          <option>en</option>
          <option>es</option>
          <option>fr</option>
        </select>

        { this.props.topic.viewerCanAddSynonym && this.renderAddButton() }
      </div>
    </dl>
  )
}

export const UnwrappedSynonyms = Synonyms

export default createFragmentContainer(Synonyms, graphql`
  fragment Synonyms_topic on Topic {
    id
    viewerCanDeleteSynonym
    viewerCanAddSynonym

    synonyms(first: 100) @connection(key: "Synonyms_synonyms") {
      edges {
        node {
          id
          name

          ...Synonym_synonym
        }
      }
    }
  }
`)
