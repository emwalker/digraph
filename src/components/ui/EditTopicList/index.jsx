// @flow
import React, { Component } from 'react'
import AsyncSelect from 'react-select/async'
import debounce from 'es6-promise-debounce'

import type { Option } from 'components/types'
import colourStyles from './colourStyles'

/* eslint react/no-unused-state: 0 */

type TopicOption = {
  +value: string,
  +label: string,
  // +color: string,
}

type Edge = {
  +node: ?TopicOption,
}

type TopicConnection = {
  +edges: ?$ReadOnlyArray<?Edge>,
}

const color = '#0366d6'

const makeOption = (edge: ?Edge): Option => (
  edge && edge.node
    ? ({ ...edge.node, color })
    : { value: 'missing', label: '<missing>', color: '' }
)

function makeOptions<T: TopicConnection>(conn: T): Option[] {
  return conn.edges ? conn.edges.map(makeOption) : []
}

type Props = {
  loadOptions: (string) => Promise<Option[]>,
  selectedTopics: Option[],
  updateTopics: Function,
}

type State = {
  inputValue: string,
  selectedTopics: Option[],
}

class EditTopicList extends Component<Props, State> {
  loadOptions: Function

  constructor(props: Props) {
    super(props)
    this.state = {
      inputValue: '',
      selectedTopics: props.selectedTopics.map((option) => ({ ...option, color })),
    }
    this.loadOptions = debounce(this.props.loadOptions, 500)
  }

  handleInputChange = (newValue: string) => {
    const inputValue = newValue.replace(/\W/g, '')
    this.setState({ inputValue })
  }

  handleChange = (selectedTopics: Option[]) => {
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
        onChange={this.handleChange}
        onInputChange={this.handleInputChange}
        placeholder="Add a topic"
        styles={colourStyles}
        value={this.state.selectedTopics}
      />
    </div>
  )
}

export default EditTopicList
export { makeOptions }
