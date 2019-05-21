// @flow
import React, { Component } from 'react'
import Select from 'react-select/lib/Async'
import debounce from 'es6-promise-debounce'

import type { Option, TopicConnection } from 'components/types'
import colourStyles from './colourStyles'

/* eslint jsx-a11y/label-has-for: 0 */

const color = '#0366d6'

const makeOption = ({ node }) => ({ ...node, color })

const makeOptions = (conn: TopicConnection): any => conn.edges.map(makeOption)

type Props = {
  loadOptions: (string) => Promise<Option[]>,
  selectedTopics: Option[],
  updateTopics: Function,
}

type State = {
  selectedTopics: ?Option[],
}

class EditTopicList extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      selectedTopics: props.selectedTopics.map(option => ({ ...option, color })),
    }
  }

  handleChange = (selectedTopics: Option[]) => {
    this.setState({ selectedTopics }, () => {
      this.props.updateTopics(selectedTopics.map(option => option.value))
    })
  }

  render = () => (
    <div className="form-group">
      <label htmlFor="parent-topics">
        Parent topics
      </label>
      <Select
        backspaceRemovesValue={false}
        className="mt-1"
        components={{
          ClearIndicator: null,
        }}
        escapeClearsValue={false}
        id="parent-topics"
        isClearable={false}
        isMulti
        loadOptions={debounce(this.props.loadOptions, 500)}
        onChange={this.handleChange}
        placeholder="Add a topic"
        styles={colourStyles}
        value={this.state.selectedTopics}
      />
    </div>
  )
}

export default EditTopicList
export { makeOptions }
