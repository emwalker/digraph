/* eslint-disable @typescript-eslint/comma-dangle */
import React, { Component } from 'react'
import { ActionMeta } from 'react-select'
import AsyncSelect from 'react-select/async'
import debounce from 'es6-promise-debounce'

import { EditTopicForm_topic } from '__generated__/EditTopicForm_topic.graphql'
import { TopicOption } from 'components/types'
import colourStyles from './colourStyles'

/* eslint react/no-unused-state: 0 */

type SynonymMatches = EditTopicForm_topic['availableTopics']['synonymMatches']
type SelectedTopics = ({ label: string, value: string } | null)[]

const color = '#0366d6'

export const makeOptions = (matches: SynonymMatches | SelectedTopics): TopicOption[] => {
  console.log('matches', matches)
  return matches
    ? (
      matches.map((match) => match
        ? { value: match.value, label: match.label, color }
        : { value: 'missing', label: '<missing>', color: '' }
      ) as TopicOption[]
    )
    : []
}

type LoadOptionsType = (str: string) => Promise<readonly TopicOption[]>

type Props = {
  loadOptions: LoadOptionsType,
  selectedTopics: readonly TopicOption[],
  updateTopics: (topics: string[]) => void,
}

type State = {
  inputValue: string,
  selectedTopics: readonly TopicOption[],
}

class EditTopicList extends Component<Props, State> {
  loadOptions: LoadOptionsType

  constructor(props: Props) {
    super(props)
    this.state = {
      inputValue: '',
      selectedTopics: props.selectedTopics.map((option: TopicOption) => ({ ...option, color })),
    }
    this.loadOptions = debounce(this.props.loadOptions, 500)
  }

  onInputChange = (newValue: string) => {
    const inputValue = newValue.replace(/\W/g, '')
    this.setState({ inputValue })
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  onChange = (selectedTopics: readonly TopicOption[], action: ActionMeta<TopicOption>) => {
    this.setState({ selectedTopics }, () => {
      this.props.updateTopics(selectedTopics.map((option: TopicOption) => option.value))
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
          ClearIndicator: undefined,
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
