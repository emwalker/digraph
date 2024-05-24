import React, { useCallback, useState } from 'react'
import AsyncSelect from 'react-select/async'
import debounce from 'es6-promise-debounce'

import { TopicOption } from 'components/types'
import colourStyles from './colourStyles'

type SelectedTopics = readonly ({ label: string, value: string } | null)[]

const color = '#0366d6'

export const makeOptions = (matches: SelectedTopics): readonly TopicOption[] => {
  return matches.map((match) => match
    ? { value: match.value, label: match.label, color }
    : { value: 'missing', label: '<missing>', color: '' },
  ) as TopicOption[]
}

type LoadOptionsType = (str: string) => Promise<readonly TopicOption[]>

type Props = {
  loadOptions: LoadOptionsType,
  selectedTopics: readonly TopicOption[],
  updateTopics: (topicIds: string[]) => void,
}

export default function EditParentTopicList(props: Props) {
  const loadOptions = debounce(props.loadOptions, 500)
  const [selectedTopics, setSelectedTopics] = useState(props.selectedTopics)
  const updateTopics = props.updateTopics

  const onChange = useCallback((topics: readonly TopicOption[]) => {
    setSelectedTopics(topics)
    updateTopics(topics.map((option: TopicOption) => option.value))
  }, [setSelectedTopics])

  return (
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
        defaultOptions={selectedTopics}
        escapeClearsValue={false}
        id="parent-topics"
        isClearable={false}
        isMulti
        loadOptions={loadOptions}
        onChange={onChange}
        placeholder="Add a topic"
        styles={colourStyles}
        value={selectedTopics}
      />
    </div>
  )
}
