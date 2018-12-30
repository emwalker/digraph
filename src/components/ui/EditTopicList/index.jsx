// @flow
import React, { Component } from 'react'
import Select from 'react-select'

import type { TopicType } from 'components/types'
import colourStyles from './colourStyles'

type Option = {
  value: string,
  label: string,
  color: string,
}

const color = '#0366d6'

type Props = {
  availableTopics: TopicType[],
  selectedTopics: TopicType[],
  updateTopics: Function,
}

type State = {
  selectedTopics: ?Option[],
}

class EditTopicList extends Component<Props, State> {
  constructor(props: Props) {
    super(props)
    this.state = {
      selectedTopics: props.selectedTopics.map(topic =>
        ({ value: topic.id, label: topic.name, color })),
    }
  }

  get options(): Option[] {
    return this.props.availableTopics.map(topic => (
      { value: topic.id, label: topic.name, color }
    ))
  }

  handleChange = (selectedTopics: Option[]) => {
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
