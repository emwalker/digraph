/* eslint-disable @typescript-eslint/comma-dangle */
import React, { Component } from 'react'
import { OptionsType, ActionMeta } from 'react-select'
import AsyncSelect from 'react-select/async'
import debounce from 'es6-promise-debounce'

import { TopicOption, Connection, Edge, liftEdges } from 'components/types'
import colourStyles from './colourStyles'

/* eslint react/no-unused-state: 0 */

const color = '#0366d6'

const makeOption = <T,>(edge: Edge<T>) => (
  edge?.node
    ? ({ ...edge.node, color })
    : { value: 'missing', label: '<missing>', color: '' }
)

export const makeOptions = <T,>(conn: Connection<T>) => liftEdges(conn).map(makeOption)

type LoadOptionsType = (str: string) => Promise<OptionsType<TopicOption>>

type Props = {
  loadOptions: LoadOptionsType,
  selectedTopics: OptionsType<TopicOption>,
  updateTopics: (topics: string[]) => void,
}

type State = {
  inputValue: string,
  selectedTopics: OptionsType<TopicOption>,
}

class EditTopicList extends Component<Props, State> {
  loadOptions: LoadOptionsType

  constructor(props: Props) {
    super(props)
    this.state = {
      inputValue: '',
      selectedTopics: props.selectedTopics.map((option) => ({ ...option, color })),
    }
    this.loadOptions = debounce(this.props.loadOptions, 500)
  }

  onInputChange = (newValue: string) => {
    const inputValue = newValue.replace(/\W/g, '')
    this.setState({ inputValue })
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  onChange = (selectedTopics: OptionsType<TopicOption>, action: ActionMeta<TopicOption>) => {
    this.setState({ selectedTopics }, () => {
      this.props.updateTopics(selectedTopics.map((option) => option.value))
    })
  }

  render = () => (
    <div className="form-group">
      <label htmlFor="parent-topics">
        Parent topics
      </label>
      <AsyncSelect
        backspaceRemovesValue={false}
        cacheOptions={false}
        className="mt-1"
        components={{
          ClearIndicator: null,
        }}
        defaultOptions={this.state.selectedTopics}
        escapeClearsValue={false}
        id="parent-topics"
        isClearable={false}
        isMulti
        loadOptions={this.loadOptions}
        onChange={this.onChange}
        onInputChange={this.onInputChange}
        placeholder="Add a topic"
        styles={colourStyles}
        value={this.state.selectedTopics}
      />
    </div>
  )
}

export default EditTopicList
