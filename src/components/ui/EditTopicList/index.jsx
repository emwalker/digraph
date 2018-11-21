// @flow
import React, { Component } from 'react'
import Select from 'react-select'

import colourStyles from './colourStyles'

type Props = {
  availableTopics: Object[],
  selectedTopics: Object[],
  updateTopics: Function,
}

const color = '#0366d6'

class EditTopicList extends Component<Props> {
  constructor(props: Props) {
    super(props)
    this.state = {
      selectedTopics: props.selectedTopics.map(topic =>
        ({ value: topic.id, label: topic.name, color })),
    }
  }

  get options() {
    return this.props.availableTopics.map(topic => (
      { value: topic.id, label: topic.name, color }
    ))
  }

  handleChange = (selectedTopics) => {
    this.setState({ selectedTopics }, () => {
      this.props.updateTopics(selectedTopics.map(option => option.value))
    })
  }

  render = () => (
    <div className="pt-3">
      <Select
        isMulti
        onChange={this.handleChange}
        options={this.options}
        placeholder="Add a topic"
        styles={colourStyles}
        value={this.state.selectedTopics}
      />
    </div>
  )
}

export default EditTopicList
