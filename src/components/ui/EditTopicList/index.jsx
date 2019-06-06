// @flow
import React, { Component } from 'react'
import AsyncSelect from 'react-select/async'
import debounce from 'es6-promise-debounce'

import type { Option, TopicConnection } from 'components/types'
import colourStyles from './colourStyles'

/* eslint react/no-unused-state: 0 */

const color = '#0366d6'

const makeOption = ({ node }) => ({ ...node, color })

const makeOptions = (conn: TopicConnection): any => conn.edges.map(makeOption)

type Props = {
  loadOptions: (string) => Promise<Option[]>,
  selectedTopics: Option[],
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
      selectedTopics: props.selectedTopics.map(option => ({ ...option, color })),
    }
    this.loadOptions = debounce(this.props.loadOptions, 500)
  }

  handleInputChange = (newValue: string) => {
    const inputValue = newValue.replace(/\W/g, '')
    this.setState({ inputValue })
  }

  render = () => (
    <div className="form-group">
      <label htmlFor="parent-topics">
        Parent topics
      </label>
      <AsyncSelect
        backspaceRemovesValue={false}
        cacheOptions
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
