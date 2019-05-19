import React from 'react'
import { shallow } from 'enzyme'
import Select from 'react-select/lib/Async'

import EditTopicList from './index'

const loadOptions = jest.fn()
const updateTopics = jest.fn()

const props = {
  loadOptions,
  selectedTopics: [],
  updateTopics,
}

describe('<EditTopicList />', () => {
  const wrapper = shallow(<EditTopicList {...props} />)
  const select = () => wrapper.find(Select)

  it('renders', () => {
    expect(wrapper).toMatchSnapshot()
  })

  describe('clearing of values', () => {
    it('passes backspaceRemovesValue=false', () => {
      expect(select().prop('backspaceRemovesValue')).toBeFalsy()
    })

    it('passes isClearable=false', () => {
      expect(select().prop('isClearable')).toBeFalsy()
    })

    it('passes escapeClearsValue=false', () => {
      expect(select().prop('escapeClearsValue')).toBeFalsy()
    })

    it('disables the ClearIndicator component', () => {
      expect(select().prop('components')).toEqual({ ClearIndicator: null })
    })
  })
})
